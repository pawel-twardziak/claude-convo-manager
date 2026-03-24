#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { execSync } from 'node:child_process';
import { createInterface } from 'node:readline';

// --- Helpers ---

function exec(cmd, opts = {}) {
	return execSync(cmd, { encoding: 'utf8', ...opts }).trim();
}

function ask(question) {
	const rl = createInterface({ input: process.stdin, output: process.stdout });
	return new Promise((resolve) => {
		rl.question(question, (answer) => {
			rl.close();
			resolve(answer.trim().toLowerCase());
		});
	});
}

// --- Git helpers ---

function getLatestTag() {
	try {
		return exec('git describe --tags --abbrev=0');
	} catch {
		return null;
	}
}

function getPreviousTag(currentTag) {
	try {
		const tags = exec('git tag --sort=-v:refnum').split('\n').filter(Boolean);
		const idx = tags.indexOf(currentTag);
		return idx >= 0 && idx + 1 < tags.length ? tags[idx + 1] : null;
	} catch {
		return null;
	}
}

// --- Conventional commit parsing ---

function parseCommits(range) {
	const separator = '---COMMIT-END---';
	let raw;
	try {
		raw = exec(`git log ${range} --format=%H%n%s%n%b%n${separator}`);
	} catch {
		return [];
	}

	if (!raw) return [];

	const commits = [];
	const entries = raw.split(separator).filter((e) => e.trim());

	for (const entry of entries) {
		const lines = entry.trim().split('\n');
		if (lines.length < 2) continue;

		const hash = lines[0];
		const subject = lines[1];
		const body = lines.slice(2).join('\n').trim();

		// Skip release commits
		if (subject.startsWith('release:')) continue;

		// Parse conventional commit: type(scope)!: description
		const match = subject.match(/^(\w+)(?:\(([^)]*)\))?(!)?:\s*(.+)$/);

		if (match) {
			commits.push({
				hash: hash.substring(0, 7),
				type: match[1],
				scope: match[2] || null,
				breaking: !!match[3] || /^BREAKING[ -]CHANGE/m.test(body),
				description: match[4],
				body
			});
		} else {
			commits.push({
				hash: hash.substring(0, 7),
				type: 'other',
				scope: null,
				breaking: false,
				description: subject,
				body
			});
		}
	}

	return commits;
}

// --- Version logic ---

function determineBump(commits) {
	let hasBreaking = false;
	let hasFeature = false;

	for (const commit of commits) {
		if (commit.breaking) hasBreaking = true;
		if (commit.type === 'feat') hasFeature = true;
	}

	if (hasBreaking) return 'major';
	if (hasFeature) return 'minor';
	return 'patch';
}

function bumpVersion(version, bump) {
	const [major, minor, patch] = version.split('.').map(Number);
	switch (bump) {
		case 'major':
			return `${major + 1}.0.0`;
		case 'minor':
			return `${major}.${minor + 1}.0`;
		case 'patch':
			return `${major}.${minor}.${patch + 1}`;
		default:
			throw new Error(`Invalid bump type: ${bump}`);
	}
}

// --- Release notes generation ---

function generateReleaseNotes(commits) {
	const typeLabels = {
		feat: 'Features',
		fix: 'Bug Fixes',
		docs: 'Documentation',
		style: 'Styles',
		refactor: 'Refactoring',
		perf: 'Performance',
		test: 'Tests',
		build: 'Build',
		ci: 'CI',
		chore: 'Chores',
		other: 'Other Changes'
	};

	const typeOrder = [
		'feat',
		'fix',
		'perf',
		'refactor',
		'docs',
		'style',
		'test',
		'build',
		'ci',
		'chore',
		'other'
	];

	// Group commits
	const groups = {};
	const breaking = [];

	for (const commit of commits) {
		if (commit.breaking) breaking.push(commit);
		const type = commit.type;
		if (!groups[type]) groups[type] = [];
		groups[type].push(commit);
	}

	let notes = '';

	// Breaking changes first
	if (breaking.length > 0) {
		notes += '## :warning: BREAKING CHANGES\n\n';
		for (const c of breaking) {
			const scope = c.scope ? `**${c.scope}:** ` : '';
			notes += `- ${scope}${c.description} (${c.hash})\n`;
			const breakingDetail = c.body.match(/^BREAKING[ -]CHANGE:\s*(.+)/m);
			if (breakingDetail) {
				notes += `  > ${breakingDetail[1]}\n`;
			}
		}
		notes += '\n';
	}

	// Grouped by type
	for (const type of typeOrder) {
		if (!groups[type] || groups[type].length === 0) continue;
		const label = typeLabels[type] || type;
		notes += `## ${label}\n\n`;
		for (const c of groups[type]) {
			const scope = c.scope ? `**${c.scope}:** ` : '';
			notes += `- ${scope}${c.description} (${c.hash})\n`;
		}
		notes += '\n';
	}

	return notes.trim();
}

// --- Notes-only mode (for CI) ---
// Usage: node scripts/release.mjs --notes <from-tag> [to-ref]

if (process.argv[2] === '--notes') {
	const fromTag = process.argv[3];
	const toRef = process.argv[4] || 'HEAD';

	if (!fromTag) {
		console.error('Usage: node scripts/release.mjs --notes <from-tag> [to-ref]');
		process.exit(1);
	}

	const commits = parseCommits(`${fromTag}..${toRef}`);
	if (commits.length === 0) {
		console.log('No notable changes.');
	} else {
		console.log(generateReleaseNotes(commits));
	}
	process.exit(0);
}

// --- Interactive release flow ---

async function main() {
	// Check for clean working tree
	const status = exec('git status --porcelain');
	if (status) {
		console.error('Working tree is not clean. Commit or stash changes first.');
		process.exit(1);
	}

	// Detect latest version
	const latestTag = getLatestTag();
	if (!latestTag) {
		console.error('No existing tags found. Create an initial release manually first.');
		process.exit(1);
	}

	const latestVersion = latestTag.replace(/^v/, '');
	console.log(`Latest release: ${latestTag}`);

	// Parse commits since last release
	const commits = parseCommits(`${latestTag}..HEAD`);

	if (commits.length === 0) {
		console.error('No commits since last release. Nothing to release.');
		process.exit(1);
	}

	console.log(`Found ${commits.length} commit(s) since ${latestTag}\n`);

	// Determine bump type (allow override via argument)
	const overrideBump = process.argv[2];
	if (overrideBump && !['major', 'minor', 'patch'].includes(overrideBump)) {
		console.error(`Invalid bump type: "${overrideBump}" — expected: major, minor, or patch`);
		process.exit(1);
	}

	const autoBump = determineBump(commits);
	const bump = overrideBump || autoBump;
	const newVersion = bumpVersion(latestVersion, bump);
	const tag = `v${newVersion}`;

	if (overrideBump && overrideBump !== autoBump) {
		console.log(`Auto-detected bump: ${autoBump} (overridden to: ${bump})`);
	} else {
		console.log(`Detected bump type: ${bump}`);
	}

	console.log(`Version: ${latestVersion} → ${newVersion}\n`);

	// Generate and preview release notes
	const notes = generateReleaseNotes(commits);
	console.log('--- Release Notes ---\n');
	console.log(notes);
	console.log('\n--- End Release Notes ---\n');

	// Confirm
	const answer = await ask(`Proceed with release ${tag}? (y/N) `);
	if (answer !== 'y') {
		console.log('Aborted.');
		process.exit(0);
	}

	// Check tag doesn't already exist
	try {
		exec(`git rev-parse ${tag}`);
		console.error(`Tag ${tag} already exists`);
		process.exit(1);
	} catch {
		// Good — tag doesn't exist
	}

	// Bump package.json
	console.log('\nBumping versions...');
	const pkg = JSON.parse(readFileSync('package.json', 'utf8'));
	pkg.version = newVersion;
	writeFileSync('package.json', JSON.stringify(pkg, null, '\t') + '\n');

	// Bump tauri.conf.json
	const tauri = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf8'));
	tauri.version = newVersion;
	writeFileSync('src-tauri/tauri.conf.json', JSON.stringify(tauri, null, 2) + '\n');

	// Bump Cargo.toml
	let cargo = readFileSync('src-tauri/Cargo.toml', 'utf8');
	cargo = cargo.replace(/^version = ".*"/m, `version = "${newVersion}"`);
	writeFileSync('src-tauri/Cargo.toml', cargo);

	// Update Cargo.lock
	console.log('Updating Cargo.lock...');
	execSync('cargo generate-lockfile', { cwd: 'src-tauri', stdio: 'inherit' });

	// Commit and tag
	console.log(`Creating commit and tag ${tag}...`);
	execSync('git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml src-tauri/Cargo.lock');
	execSync(`git commit -m "release: ${tag}"`);
	execSync(`git tag -a ${tag} -m "${tag}"`);

	// Push
	const pushAnswer = await ask('Push to remote? (y/N) ');
	if (pushAnswer === 'y') {
		console.log('Pushing...');
		execSync(`git push && git push origin ${tag}`, { stdio: 'inherit' });
		console.log(`\nReleased ${tag} — GitHub Actions will build and create the draft release.`);
	} else {
		console.log(`\nDone! Created commit and tag ${tag}`);
		console.log(`Push with: git push && git push origin ${tag}`);
	}
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
