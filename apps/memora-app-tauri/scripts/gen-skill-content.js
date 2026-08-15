// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

import { readFile, writeFile, access, mkdir } from 'fs/promises'
import path from 'path'

const repoRoot = path.resolve(__dirname, '../../..')
const outFile = path.resolve(__dirname, '../lib/generated/memora-skills.ts')

const SOURCES = {
	MEMORA_API_SKILL_MD: path.join(repoRoot, 'crates/memora-core/assets/skills/memora-api/SKILL.md'),
	MEMORA_CLI_SKILL_MD: path.join(repoRoot, 'crates/memora-core/assets/skills/memora-cli/SKILL.md'),
}

const HEADER = `// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

// GENERATED FILE - do not edit by hand.
// Regenerate: bun scripts/gen-skill-content.js
`

async function exists(p) {
	try {
		await access(p)
		return true
	} catch {
		return false
	}
}

async function main() {
	let body = ''
	try {
		for (const [name, src] of Object.entries(SOURCES)) {
			const content = await readFile(src, 'utf8')
			body += `export const ${name} = ${JSON.stringify(content)};\n\n`
		}
	} catch (e) {
		if (await exists(outFile)) {
			console.warn(
				`[gen-skill-content] could not read source SKILL.md (${e.message}); keeping existing ${path.relative(repoRoot, outFile)}`,
			)
			return
		}
		throw new Error(
			`[gen-skill-content] missing source SKILL.md and no existing generated file: ${e.message}`,
		)
	}

	await mkdir(path.dirname(outFile), { recursive: true })
	await writeFile(outFile, HEADER + body, 'utf8')
	console.log(`[gen-skill-content] wrote ${path.relative(repoRoot, outFile)}`)
}

main()
