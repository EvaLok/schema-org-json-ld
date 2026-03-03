import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import phpParser from "php-parser";
import ts from "typescript";

type SchemaProperty = {
name: string;
types: string[];
nullable: boolean;
hasDefault: boolean;
};

type SchemaDefinition = {
name: string;
parent: string | null;
schemaType: string[];
propertyMap: Record<string, string>;
properties: SchemaProperty[];
};

type EnumDefinition = {
name: string;
values: Record<string, string>;
};

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "..");
const phpAstParser = new phpParser.Engine({
	parser: { extractDoc: false },
	ast: { withPositions: false },
});

function listFiles(directory: string, extension: string): string[] {
return readdirSync(directory)
.filter((file) => file.endsWith(extension))
.map((file) => path.join(directory, file));
}

function uniqueSorted(values: string[]): string[] {
return [...new Set(values)].sort();
}

function normalizePhpType(type: string): string {
const cleaned = type.trim().replace(/^\\+/, "");

if (cleaned === "int" || cleaned === "float") return "number";
if (cleaned === "bool") return "boolean";
if (cleaned === "array") return "array";

return cleaned;
}

function normalizeTsType(type: string): string {
let cleaned = type.trim().replace(/^readonly\s+/, "");

while (cleaned.startsWith("(") && cleaned.endsWith(")")) {
cleaned = cleaned.slice(1, -1).trim();
}

if (cleaned.endsWith("[]") || /^Array<.+>$/.test(cleaned)) {
return "array";
}

if (cleaned === "undefined") {
return "null";
}

return cleaned;
}

type PhpNode = {
	kind?: string;
	[key: string]: unknown;
};

function isPhpNode(value: unknown): value is PhpNode {
	return typeof value === "object" && value !== null;
}

function forEachPhpNode(node: unknown, visitor: (currentNode: PhpNode) => void): void {
	if (!node) {
		return;
	}

	if (Array.isArray(node)) {
		for (const child of node) {
			forEachPhpNode(child, visitor);
		}
		return;
	}

	if (!isPhpNode(node)) {
		return;
	}

	if (typeof node.kind === "string") {
		visitor(node);
	}

	for (const value of Object.values(node)) {
		forEachPhpNode(value, visitor);
	}
}

function parsePhpSchemaType(node: PhpNode | undefined): string[] {
	if (!node) {
		return [];
	}

	if (node.kind === "string" && typeof node.value === "string") {
		return [node.value];
	}

	if (node.kind === "array" && Array.isArray(node.items)) {
		return node.items
			.map((item) => {
				const value = isPhpNode(item) ? item.value : null;
				if (isPhpNode(value) && value.kind === "string" && typeof value.value === "string") {
					return value.value;
				}
				return null;
			})
			.filter((value): value is string => value !== null);
	}

	return [];
}

function parsePhpPropertyMap(node: PhpNode | undefined): Record<string, string> {
	const map: Record<string, string> = {};
	if (!node || node.kind !== "array" || !Array.isArray(node.items)) {
		return map;
	}

	for (const item of node.items) {
		if (!isPhpNode(item) || item.kind !== "entry") {
			continue;
		}

		const key = isPhpNode(item.key) ? item.key : null;
		const value = isPhpNode(item.value) ? item.value : null;
		if (
			key?.kind === "string" &&
			typeof key.value === "string" &&
			value?.kind === "string" &&
			typeof value.value === "string"
		) {
			map[key.value] = value.value;
		}
	}

	return map;
}

function extractPhpType(typeNode: unknown): { types: string[]; nullable: boolean } {
	if (!isPhpNode(typeNode)) {
		return { types: [], nullable: false };
	}

	if (typeNode.kind === "uniontype" && Array.isArray(typeNode.types)) {
		const nested = typeNode.types.map((nestedType) => extractPhpType(nestedType));
		return {
			types: uniqueSorted(nested.flatMap((entry) => entry.types)),
			nullable: nested.some((entry) => entry.nullable),
		};
	}

	let rawType = "";
	if (typeNode.kind === "typereference" && typeof typeNode.name === "string") {
		rawType = typeNode.name;
	}
	if (typeNode.kind === "name" && typeof typeNode.name === "string") {
		rawType = typeNode.name;
	}

	const nullable = rawType === "null" || typeNode.nullable === true;
	const normalized = rawType ? normalizePhpType(rawType) : "";
	return {
		types: normalized && normalized !== "null" ? [normalized] : [],
		nullable,
	};
}

function getTsPropertyName(node: ts.PropertyName): string | null {
	if (ts.isIdentifier(node) || ts.isStringLiteral(node) || ts.isNumericLiteral(node)) {
		return node.text;
	}
	return null;
}

function parseTsSchemaType(node: ts.Expression | undefined): string[] {
	if (!node) {
		return [];
	}

	if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) {
		return [node.text];
	}

	if (ts.isArrayLiteralExpression(node)) {
		return node.elements
			.filter(
				(element): element is ts.StringLiteral | ts.NoSubstitutionTemplateLiteral =>
					ts.isStringLiteral(element) || ts.isNoSubstitutionTemplateLiteral(element),
			)
			.map((element) => element.text);
	}

	return [];
}

function parseTsPropertyMap(node: ts.Expression | undefined): Record<string, string> {
	const map: Record<string, string> = {};
	if (!node || !ts.isObjectLiteralExpression(node)) {
		return map;
	}

	for (const property of node.properties) {
		if (!ts.isPropertyAssignment(property)) {
			continue;
		}

		const key = getTsPropertyName(property.name);
		const value = property.initializer;
		if (!key) {
			continue;
		}

		if (ts.isStringLiteral(value) || ts.isNoSubstitutionTemplateLiteral(value)) {
			map[key] = value.text;
		}
	}

	return map;
}

function extractTsTypes(typeNode: ts.TypeNode | undefined): { types: string[]; nullable: boolean } {
	if (!typeNode) {
		return { types: [], nullable: false };
	}

	if (ts.isUnionTypeNode(typeNode)) {
		const nested = typeNode.types.map((nestedType) => extractTsTypes(nestedType));
		return {
			types: uniqueSorted(nested.flatMap((entry) => entry.types)),
			nullable: nested.some((entry) => entry.nullable),
		};
	}

	const normalized = normalizeTsType(typeNode.getText());
	return {
		types: normalized && normalized !== "null" ? [normalized] : [],
		nullable: normalized === "null",
	};
}

function parsePhpSchemaFile(filePath: string): SchemaDefinition | null {
	const content = readFileSync(filePath, "utf8");
	const ast = phpAstParser.parseCode(content, filePath);
	let classNode: PhpNode | null = null;

	forEachPhpNode(ast, (node) => {
		if (!classNode && node.kind === "class") {
			classNode = node;
		}
	});

	if (
		!classNode ||
		!isPhpNode(classNode.name) ||
		typeof classNode.name.name !== "string"
	) {
		return null;
	}

	const name = classNode.name.name;
	const parentClass =
		isPhpNode(classNode.extends) && typeof classNode.extends.name === "string"
			? classNode.extends.name
			: "TypedSchema";
	const propertyMap: Record<string, string> = {};
	let schemaType: string[] = [];
	const properties: SchemaProperty[] = [];

	const body = Array.isArray(classNode.body) ? classNode.body : [];
	for (const member of body) {
		if (!isPhpNode(member)) {
			continue;
		}

		if (member.kind === "classconstant" && Array.isArray(member.constants)) {
			for (const constant of member.constants) {
				if (!isPhpNode(constant) || !isPhpNode(constant.name)) {
					continue;
				}

				const constantName = constant.name.name;
				if (constantName === "A_SCHEMA_TYPE") {
					schemaType = parsePhpSchemaType(isPhpNode(constant.value) ? constant.value : undefined);
				}
				if (constantName === "PROPERTY_MAP") {
					Object.assign(
						propertyMap,
						parsePhpPropertyMap(isPhpNode(constant.value) ? constant.value : undefined),
					);
				}
			}
		}

		if (
			member.kind === "method" &&
			isPhpNode(member.name) &&
			member.name.name === "__construct" &&
			Array.isArray(member.arguments)
		) {
			for (const argument of member.arguments) {
				if (!isPhpNode(argument) || !isPhpNode(argument.name) || typeof argument.flags !== "number") {
					continue;
				}
				if (argument.flags === 0) {
					continue;
				}

				const parsedType = extractPhpType(argument.type);
				properties.push({
					name: argument.name.name,
					types: uniqueSorted(parsedType.types),
					nullable: parsedType.nullable,
					hasDefault: argument.value !== null,
				});
			}
		}
	}

	return {
		name,
		parent: parentClass === "TypedSchema" ? null : parentClass,
		schemaType,
		propertyMap,
		properties,
	};
}

function parseTsSchemaFile(filePath: string): SchemaDefinition | null {
	const content = readFileSync(filePath, "utf8");
	const sourceFile = ts.createSourceFile(
		filePath,
		content,
		ts.ScriptTarget.Latest,
		true,
	);
	let classNode: ts.ClassDeclaration | null = null;
	const interfaces = new Map<string, ts.InterfaceDeclaration>();

	for (const statement of sourceFile.statements) {
		if (ts.isInterfaceDeclaration(statement)) {
			interfaces.set(statement.name.text, statement);
		}
		if (ts.isClassDeclaration(statement) && statement.name && !classNode) {
			classNode = statement;
		}
	}

	if (!classNode?.name) {
		return null;
	}

	const name = classNode.name.text;
	const parentType = classNode.heritageClauses
		?.find((clause) => clause.token === ts.SyntaxKind.ExtendsKeyword)
		?.types.at(0);
	const parentClass = parentType ? parentType.expression.getText(sourceFile) : "TypedSchema";
	let schemaType: string[] = [];
	let propertyMap: Record<string, string> = {};

	for (const member of classNode.members) {
		if (
			!ts.isPropertyDeclaration(member) ||
			!member.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.StaticKeyword)
		) {
			continue;
		}

		const memberName = member.name && ts.isIdentifier(member.name) ? member.name.text : null;
		if (!memberName) {
			continue;
		}

		if (memberName === "schemaType" || memberName === "SCHEMA_TYPE") {
			schemaType = parseTsSchemaType(member.initializer);
		}
		if (memberName === "propertyMap") {
			propertyMap = parseTsPropertyMap(member.initializer);
		}
	}

	const optionsInterface = interfaces.get(`${name}Options`);
	const properties: SchemaProperty[] = [];
	if (optionsInterface) {
		for (const member of optionsInterface.members) {
			if (!ts.isPropertySignature(member) || !member.name) {
				continue;
			}

			const propName = getTsPropertyName(member.name);
			if (!propName) {
				continue;
			}

			const parsedType = extractTsTypes(member.type);
			properties.push({
				name: propName,
				types: uniqueSorted(parsedType.types),
				nullable: member.questionToken !== undefined || parsedType.nullable,
				hasDefault: member.questionToken !== undefined,
			});
		}
	}

	return {
		name,
		parent: parentClass === "TypedSchema" ? null : parentClass,
		schemaType,
		propertyMap,
		properties,
	};
}

function parsePhpEnumFile(filePath: string): EnumDefinition | null {
	const content = readFileSync(filePath, "utf8");
	const ast = phpAstParser.parseCode(content, filePath);
	let enumNode: PhpNode | null = null;

	forEachPhpNode(ast, (node) => {
		if (!enumNode && node.kind === "enum") {
			enumNode = node;
		}
	});

	if (!enumNode || !isPhpNode(enumNode.name) || typeof enumNode.name.name !== "string") {
		return null;
	}

	const values: Record<string, string> = {};
	const body = Array.isArray(enumNode.body) ? enumNode.body : [];
	for (const member of body) {
		if (!isPhpNode(member) || member.kind !== "enumcase" || !isPhpNode(member.name)) {
			continue;
		}

		const value = isPhpNode(member.value) ? member.value : null;
		if (value?.kind === "string" && typeof value.value === "string") {
			values[member.name.name] = value.value;
		}
	}

	return { name: enumNode.name.name, values };
}

function parseTsEnumFile(filePath: string): EnumDefinition | null {
	const content = readFileSync(filePath, "utf8");
	const sourceFile = ts.createSourceFile(
		filePath,
		content,
		ts.ScriptTarget.Latest,
		true,
	);
	const enumNode = sourceFile.statements.find((statement) =>
		ts.isEnumDeclaration(statement),
	);

	if (!enumNode || !ts.isEnumDeclaration(enumNode)) {
		return null;
	}

	const values: Record<string, string> = {};
	for (const member of enumNode.members) {
		if (!member.initializer) {
			continue;
		}

		if (
			ts.isStringLiteral(member.initializer) ||
			ts.isNoSubstitutionTemplateLiteral(member.initializer)
		) {
			values[member.name.getText(sourceFile)] = member.initializer.text;
		}
	}

	return { name: enumNode.name.text, values };
}

function parsePhpSchemas(directory: string): Map<string, SchemaDefinition> {
const classes = new Map<string, SchemaDefinition>();

for (const filePath of listFiles(directory, ".php")) {
const schema = parsePhpSchemaFile(filePath);
if (schema) {
classes.set(schema.name, schema);
}
}

return classes;
}

function parseTsSchemas(directory: string): Map<string, SchemaDefinition> {
const classes = new Map<string, SchemaDefinition>();

for (const filePath of listFiles(directory, ".ts")) {
const schema = parseTsSchemaFile(filePath);
if (schema) {
classes.set(schema.name, schema);
}
}

return classes;
}

function parsePhpEnums(directory: string): Map<string, EnumDefinition> {
const enums = new Map<string, EnumDefinition>();

for (const filePath of listFiles(directory, ".php")) {
const parsed = parsePhpEnumFile(filePath);
if (parsed) {
enums.set(parsed.name, parsed);
}
}

return enums;
}

function parseTsEnums(directory: string): Map<string, EnumDefinition> {
const enums = new Map<string, EnumDefinition>();

for (const filePath of listFiles(directory, ".ts")) {
const parsed = parseTsEnumFile(filePath);
if (parsed) {
enums.set(parsed.name, parsed);
}
}

return enums;
}

function areStringArraysEqual(a: string[], b: string[]): boolean {
if (a.length !== b.length) {
return false;
}

const sortedA = [...a].sort();
const sortedB = [...b].sort();

return sortedA.every((value, index) => value === sortedB[index]);
}

function comparePropertyMaps(
a: Record<string, string>,
b: Record<string, string>,
): string[] {
const discrepancies: string[] = [];
const keys = new Set([...Object.keys(a), ...Object.keys(b)]);

for (const key of keys) {
if (!(key in a)) {
discrepancies.push(
`TS has propertyMap key '${key}' -> '${b[key]}' but PHP is missing it`,
);
continue;
}

if (!(key in b)) {
discrepancies.push(
`PHP has propertyMap key '${key}' -> '${a[key]}' but TS is missing it`,
);
continue;
}

if (a[key] !== b[key]) {
discrepancies.push(
`propertyMap mismatch for '${key}': PHP='${a[key]}' TS='${b[key]}'`,
);
}
}

return discrepancies;
}

function compareSchemas(
phpSchemas: Map<string, SchemaDefinition>,
tsSchemas: Map<string, SchemaDefinition>,
): { perfectMatches: number; discrepancies: Map<string, string[]> } {
const discrepancies = new Map<string, string[]>();
let perfectMatches = 0;

const allClassNames = new Set([...phpSchemas.keys(), ...tsSchemas.keys()]);
for (const className of allClassNames) {
const classDiscrepancies: string[] = [];
const phpSchema = phpSchemas.get(className);
const tsSchema = tsSchemas.get(className);

if (!phpSchema) {
classDiscrepancies.push("Class exists in TS but is missing in PHP");
discrepancies.set(className, classDiscrepancies);
continue;
}

if (!tsSchema) {
classDiscrepancies.push("Class exists in PHP but is missing in TS");
discrepancies.set(className, classDiscrepancies);
continue;
}

if (phpSchema.parent !== tsSchema.parent) {
classDiscrepancies.push(
`Parent mismatch: PHP='${phpSchema.parent ?? "TypedSchema"}' TS='${tsSchema.parent ?? "TypedSchema"}'`,
);
}

if (!areStringArraysEqual(phpSchema.schemaType, tsSchema.schemaType)) {
classDiscrepancies.push(
`SCHEMA_TYPE mismatch: PHP=${JSON.stringify(phpSchema.schemaType)} TS=${JSON.stringify(tsSchema.schemaType)}`,
);
}

classDiscrepancies.push(
...comparePropertyMaps(phpSchema.propertyMap, tsSchema.propertyMap),
);

const phpProperties = new Map(
phpSchema.properties.map((prop) => [prop.name, prop]),
);
const tsProperties = new Map(tsSchema.properties.map((prop) => [prop.name, prop]));
const allPropertyNames = new Set([
...phpProperties.keys(),
...tsProperties.keys(),
]);

for (const propertyName of allPropertyNames) {
const phpProperty = phpProperties.get(propertyName);
const tsProperty = tsProperties.get(propertyName);

if (!phpProperty) {
classDiscrepancies.push(
`TS has property '${propertyName}' but PHP is missing it`,
);
continue;
}

if (!tsProperty) {
classDiscrepancies.push(
`PHP has property '${propertyName}' but TS is missing it`,
);
continue;
}

if (!areStringArraysEqual(phpProperty.types, tsProperty.types)) {
classDiscrepancies.push(
`Type mismatch for '${propertyName}': PHP=${phpProperty.types.join("|")} TS=${tsProperty.types.join("|")}`,
);
}

if (phpProperty.nullable !== tsProperty.nullable) {
classDiscrepancies.push(
`Nullability mismatch for '${propertyName}': PHP=${phpProperty.nullable} TS=${tsProperty.nullable}`,
);
}
}

if (classDiscrepancies.length > 0) {
discrepancies.set(className, classDiscrepancies);
} else {
perfectMatches += 1;
}
}

return { perfectMatches, discrepancies };
}

function compareEnums(
phpEnums: Map<string, EnumDefinition>,
tsEnums: Map<string, EnumDefinition>,
): Map<string, string[]> {
const discrepancies = new Map<string, string[]>();
const enumNames = new Set([...phpEnums.keys(), ...tsEnums.keys()]);

for (const enumName of enumNames) {
const enumDiscrepancies: string[] = [];
const phpEnum = phpEnums.get(enumName);
const tsEnum = tsEnums.get(enumName);

if (!phpEnum) {
enumDiscrepancies.push("Enum exists in TS but is missing in PHP");
discrepancies.set(enumName, enumDiscrepancies);
continue;
}

if (!tsEnum) {
enumDiscrepancies.push("Enum exists in PHP but is missing in TS");
discrepancies.set(enumName, enumDiscrepancies);
continue;
}

const caseNames = new Set([
...Object.keys(phpEnum.values),
...Object.keys(tsEnum.values),
]);
for (const caseName of caseNames) {
if (!(caseName in phpEnum.values)) {
enumDiscrepancies.push(
`TS has enum case '${caseName}' but PHP is missing it`,
);
continue;
}

if (!(caseName in tsEnum.values)) {
enumDiscrepancies.push(
`PHP has enum case '${caseName}' but TS is missing it`,
);
continue;
}

if (phpEnum.values[caseName] !== tsEnum.values[caseName]) {
enumDiscrepancies.push(
`Enum value mismatch for '${caseName}': PHP='${phpEnum.values[caseName]}' TS='${tsEnum.values[caseName]}'`,
);
}
}

if (enumDiscrepancies.length > 0) {
discrepancies.set(enumName, enumDiscrepancies);
}
}

return discrepancies;
}

export function run(): number {
const phpSchemaDir = path.join(repoRoot, "php", "src", "v1", "Schema");
const tsSchemaDir = path.join(repoRoot, "ts", "src", "schema");
const phpEnumDir = path.join(repoRoot, "php", "src", "v1", "Enum");
const tsEnumDir = path.join(repoRoot, "ts", "src", "enum");

const phpSchemas = parsePhpSchemas(phpSchemaDir);
const tsSchemas = parseTsSchemas(tsSchemaDir);
const phpEnums = parsePhpEnums(phpEnumDir);
const tsEnums = parseTsEnums(tsEnumDir);

const schemaComparison = compareSchemas(phpSchemas, tsSchemas);
const enumDiscrepancies = compareEnums(phpEnums, tsEnums);
const totalDiscrepancies =
schemaComparison.discrepancies.size + enumDiscrepancies.size;

console.log("=== Schema Parity Report ===");
console.log(`PHP classes: ${phpSchemas.size}`);
console.log(`TS classes: ${tsSchemas.size}`);
console.log(`Enums: ${phpEnums.size}/${tsEnums.size}`);
console.log("");

console.log(`✓ ${schemaComparison.perfectMatches} classes match perfectly`);
if (schemaComparison.discrepancies.size > 0) {
console.log(
`✗ ${schemaComparison.discrepancies.size} classes have discrepancies:`,
);
console.log("");

for (const [className, classDiscrepancies] of [
...schemaComparison.discrepancies.entries(),
].sort(([a], [b]) => a.localeCompare(b))) {
console.log(`  ${className}:`);
for (const message of classDiscrepancies) {
console.log(`    - ${message}`);
}
console.log("");
}
}

if (enumDiscrepancies.size > 0) {
console.log("Enum discrepancies:");
console.log("");
for (const [enumName, messages] of [...enumDiscrepancies.entries()].sort(
([a], [b]) => a.localeCompare(b),
)) {
console.log(`  ${enumName}:`);
for (const message of messages) {
console.log(`    - ${message}`);
}
console.log("");
}
}

console.log("=== Summary ===");
console.log(`Total discrepancies: ${totalDiscrepancies}`);

return totalDiscrepancies === 0 ? 0 : 1;
}

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
process.exitCode = run();
}
