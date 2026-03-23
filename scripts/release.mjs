#!/usr/bin/env node
import { readFileSync, writeFileSync, execSync } from 'node:fs';

const version = process.argv[2];

if (!version) {
  console.error('Usage: node scripts/release.mjs <version>');
  console.error('Example: node scripts/release.mjs 0.2.0');
  process.exit(1);
}

if (!/^\d+\.\d+\.\d+$/.test(version)) {
  console.error(`Invalid semver: "${version}" — expected format: X.Y.Z`);
  process.exit(1);
}

const tag = `v${version}`;

// Check that tag doesn't already exist
try {
  execSync(`git rev-parse ${tag}`, { stdio: 'pipe' });
  console.error(`Tag ${tag} already exists`);
  process.exit(1);
} catch {
  // Tag doesn't exist — good
}

// Check for clean working tree
const status = execSync('git status --porcelain').toString().trim();
if (status) {
  console.error('Working tree is not clean. Commit or stash changes first.');
  process.exit(1);
}

// 1. Bump package.json
const pkg = JSON.parse(readFileSync('package.json', 'utf8'));
pkg.version = version;
writeFileSync('package.json', JSON.stringify(pkg, null, '\t') + '\n');

// 2. Bump tauri.conf.json
const tauri = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf8'));
tauri.version = version;
writeFileSync('src-tauri/tauri.conf.json', JSON.stringify(tauri, null, 2) + '\n');

// 3. Bump Cargo.toml
let cargo = readFileSync('src-tauri/Cargo.toml', 'utf8');
cargo = cargo.replace(/^version = ".*"/m, `version = "${version}"`);
writeFileSync('src-tauri/Cargo.toml', cargo);

// 4. Update Cargo.lock
console.log('Updating Cargo.lock...');
execSync('cargo generate-lockfile', { cwd: 'src-tauri', stdio: 'inherit' });

// 5. Commit and tag
console.log(`Bumping to ${version}...`);
execSync('git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml src-tauri/Cargo.lock');
execSync(`git commit -m "release: ${tag}"`);
execSync(`git tag -a ${tag} -m "${tag}"`);

console.log(`\nDone! Created commit and tag ${tag}`);
console.log(`Push with: git push && git push origin ${tag}`);