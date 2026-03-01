import { cpSync, existsSync, mkdtempSync, rmSync, unlinkSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import path from "node:path";

const scriptDir = path.dirname(decodeURIComponent(new URL(import.meta.url).pathname));
const repoRoot = path.resolve(scriptDir, "..");

const REQUIRED_EXPORTS = [
	"JsonLdGenerator",
	"Product",
	"BreadcrumbList",
	"Brand",
	"Article",
	"ItemAvailability",
];

function run(command, args, options = {}) {
	const result = spawnSync(command, args, {
		cwd: options.cwd ?? repoRoot,
		encoding: "utf8",
		stdio: "pipe",
	});

	if (result.status !== 0) {
		const error = new Error(
			`Command failed: ${command} ${args.join(" ")}\n${result.stdout}\n${result.stderr}`,
		);
		error.cause = result.error;
		throw error;
	}

	return result.stdout.trim();
}

function pass(message) {
	console.log(`✅ ${message}`);
}

function fail(message) {
	console.error(`❌ ${message}`);
}

function assertRequiredExports(pkg, kind) {
	for (const exportName of REQUIRED_EXPORTS) {
		if (!(exportName in pkg)) {
			throw new Error(`${kind} missing export: ${exportName}`);
		}
	}
}

let tempDir = null;
let tarballPath = null;
let copiedDist = false;

try {
	run("npm", ["run", "build"]);
	pass("Build completed");

	const packageDistPath = path.join(repoRoot, "dist");
	const fallbackDistPath = path.resolve(repoRoot, "..", "dist");
	if (!existsSync(packageDistPath) && existsSync(fallbackDistPath)) {
		cpSync(fallbackDistPath, packageDistPath, { recursive: true });
		copiedDist = true;
	}

	const packOutput = run("npm", ["pack"]);
	const tarballName = packOutput
		.split("\n")
		.map((line) => line.trim())
		.filter((line) => line.endsWith(".tgz"))
		.pop();

	if (tarballName === undefined) {
		throw new Error(`Unable to find tarball name in npm pack output:\n${packOutput}`);
	}

	tarballPath = path.resolve(repoRoot, tarballName);
	pass(`Packed tarball: ${tarballName}`);

	tempDir = mkdtempSync(path.join(tmpdir(), "schema-org-json-ld-verify-"));
	run("npm", ["init", "-y"], { cwd: tempDir });
	run("npm", ["install", tarballPath], { cwd: tempDir });
	pass("Installed tarball in temporary project");

	run(
		"node",
		[
			"--input-type=module",
			"-e",
			`const pkg = await import("@evabee/schema-org-json-ld"); const required = ${JSON.stringify(REQUIRED_EXPORTS)}; for (const key of required) { if (!(key in pkg)) { throw new Error("ESM missing export: " + key); } }`,
		],
		{ cwd: tempDir },
	);
	pass("ESM import validated");

	const requireFromTemp = createRequire(path.join(tempDir, "package.json"));
	const cjsPkg = requireFromTemp("@evabee/schema-org-json-ld");
	assertRequiredExports(cjsPkg, "CJS");
	pass("CJS require validated");

	const brand = new cjsPkg.Brand("Acme");
	const offer = new cjsPkg.Offer({
		url: "https://example.com/products/widget",
		priceCurrency: "USD",
		price: 19.99,
		availability: cjsPkg.ItemAvailability.InStock,
	});
	const product = new cjsPkg.Product({
		name: "Widget",
		image: ["https://example.com/products/widget.jpg"],
		description: "Demo product",
		sku: "WIDGET-1",
		offers: [offer],
		brand,
	});

	const json = cjsPkg.JsonLdGenerator.schemaToJson(product);
	const parsed = JSON.parse(json);

	if (
		parsed["@context"] !== "https://schema.org/" ||
		parsed["@type"] !== "Product" ||
		parsed.name !== "Widget" ||
		parsed.brand?.name !== "Acme" ||
		parsed.offers?.[0]?.price !== 19.99
	) {
		throw new Error("Generated JSON-LD did not contain expected values");
	}

	pass("JSON-LD generation validated");
	console.log("🎉 verify-build completed successfully");
} catch (error) {
	fail(error instanceof Error ? error.message : String(error));
	process.exitCode = 1;
} finally {
	if (tempDir !== null && existsSync(tempDir)) {
		rmSync(tempDir, { recursive: true, force: true });
	}

	if (tarballPath !== null && existsSync(tarballPath)) {
		unlinkSync(tarballPath);
	}

	if (copiedDist) {
		rmSync(path.join(repoRoot, "dist"), { recursive: true, force: true });
	}
}
