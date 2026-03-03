import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

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

function listFiles(directory: string, extension: string): string[] {
return readdirSync(directory)
.filter((file) => file.endsWith(extension))
.map((file) => path.join(directory, file));
}

function splitTopLevel(input: string, separator: string): string[] {
const parts: string[] = [];
let current = "";
let depthParen = 0;
let depthAngle = 0;
let depthSquare = 0;
let quote: '"' | "'" | null = null;

for (const char of input) {
if (quote) {
current += char;
if (char === quote) {
quote = null;
}
continue;
}

if (char === '"' || char === "'") {
quote = char;
current += char;
continue;
}

if (char === "(") depthParen += 1;
if (char === ")") depthParen -= 1;
if (char === "<") depthAngle += 1;
if (char === ">") depthAngle -= 1;
if (char === "[") depthSquare += 1;
if (char === "]") depthSquare -= 1;

if (
char === separator &&
depthParen === 0 &&
depthAngle === 0 &&
depthSquare === 0
) {
if (current.trim().length > 0) {
parts.push(current.trim());
}
current = "";
continue;
}

current += char;
}

if (current.trim().length > 0) {
parts.push(current.trim());
}

return parts;
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

function parseSchemaType(rawValue: string): string[] {
const trimmed = rawValue.trim();

if (trimmed.startsWith("[")) {
return [...trimmed.matchAll(/["']([^"']+)["']/g)].map((match) => match[1]);
}

const single = trimmed.match(/["']([^"']+)["']/);
if (!single) {
return [];
}

return [single[1]];
}

function parsePropertyMap(
rawValue: string,
assignmentOperator: "=>" | ":",
): Record<string, string> {
const map: Record<string, string> = {};
const regex =
assignmentOperator === "=>"
? /["']([^"']+)["']\s*=>\s*["']([^"']+)["']/g
: /([A-Za-z_][A-Za-z0-9_]*)\s*:\s*["']([^"']+)["']/g;

for (const match of rawValue.matchAll(regex)) {
map[match[1]] = match[2];
}

return map;
}

function parsePhpSchemaFile(filePath: string): SchemaDefinition | null {
const content = readFileSync(filePath, "utf8");
const classMatch = content.match(/class\s+(\w+)\s+extends\s+(\w+)/);
if (!classMatch) {
return null;
}

const [, name, parentClass] = classMatch;
const schemaTypeMatch = content.match(/const\s+A_SCHEMA_TYPE\s*=\s*([^;]+);/);
const propertyMapMatch = content.match(/const\s+PROPERTY_MAP\s*=\s*\[([\s\S]*?)\];/);
const constructorMatch = content.match(/function\s+__construct\s*\(([\s\S]*?)\)\s*\{/);

const propertyMap = propertyMapMatch
? parsePropertyMap(propertyMapMatch[1], "=>")
: {};
const properties: SchemaProperty[] = [];

if (constructorMatch) {
const constructorBody = constructorMatch[1].replace(/\/\*\*[\s\S]*?\*\//g, "");
const params = splitTopLevel(constructorBody, ",");

for (const param of params) {
const normalized = param.replace(/\s+/g, " ").trim();
const paramMatch = normalized.match(
/^public\s+([^$]+?)\s+\$(\w+)(?:\s*=\s*(.+))?$/,
);
if (!paramMatch) {
continue;
}

const [, rawType, propName, defaultValue] = paramMatch;
const rawTypes = splitTopLevel(rawType, "|");
const nullable = rawTypes.some((type) => type.trim() === "null");
const types = uniqueSorted(
rawTypes
.map((type) => normalizePhpType(type))
.filter((type) => type !== "null"),
);

properties.push({
name: propName,
types,
nullable,
hasDefault: defaultValue !== undefined,
});
}
}

return {
name,
parent: parentClass === "TypedSchema" ? null : parentClass,
schemaType: schemaTypeMatch ? parseSchemaType(schemaTypeMatch[1]) : [],
propertyMap,
properties,
};
}

function parseTsSchemaFile(filePath: string): SchemaDefinition | null {
const content = readFileSync(filePath, "utf8");
const classMatch = content.match(/export\s+class\s+(\w+)\s+extends\s+(\w+)/);
if (!classMatch) {
return null;
}

const [, name, parentClass] = classMatch;
const schemaTypeMatch = content.match(
/static\s+readonly\s+(?:schemaType|SCHEMA_TYPE)(?:\s*:[^=;]+)?\s*=\s*([^;]+);/,
);
const propertyMapMatch = content.match(
/static\s+readonly\s+propertyMap[^=]*=\s*\{([\s\S]*?)\};/,
);
const optionsInterfaceRegex = new RegExp(
`export\\s+interface\\s+${name}Options\\s*\\{([\\s\\S]*?)\\}`,
"m",
);
const optionsInterfaceMatch = content.match(optionsInterfaceRegex);

const propertyMap = propertyMapMatch
? parsePropertyMap(propertyMapMatch[1], ":")
: {};
const properties: SchemaProperty[] = [];

if (optionsInterfaceMatch) {
const interfaceBody = optionsInterfaceMatch[1].replace(/\/\*\*[\s\S]*?\*\//g, "");
const entries = interfaceBody
.split(";")
.map((entry) => entry.trim())
.filter(Boolean);

for (const entry of entries) {
const propMatch = entry.match(/^(\w+)(\?)?\s*:\s*([\s\S]+)$/);
if (!propMatch) {
continue;
}

const [, propName, optionalMarker, rawType] = propMatch;
const rawTypes = splitTopLevel(rawType, "|");
const nullable =
optionalMarker === "?" ||
rawTypes.some((type) => {
const normalized = type.trim();
return normalized === "null" || normalized === "undefined";
});
const types = uniqueSorted(
rawTypes
.map((type) => normalizeTsType(type))
.filter((type) => type !== "null"),
);

properties.push({
name: propName,
types,
nullable,
hasDefault: optionalMarker === "?",
});
}
}

return {
name,
parent: parentClass === "TypedSchema" ? null : parentClass,
schemaType: schemaTypeMatch ? parseSchemaType(schemaTypeMatch[1]) : [],
propertyMap,
properties,
};
}

function parsePhpEnumFile(filePath: string): EnumDefinition | null {
const content = readFileSync(filePath, "utf8");
const enumMatch = content.match(/enum\s+(\w+)\s*:\s*string\s*\{([\s\S]*?)\}/);
if (!enumMatch) {
return null;
}

const [, name, body] = enumMatch;
const values: Record<string, string> = {};

for (const match of body.matchAll(/case\s+(\w+)\s*=\s*["']([^"']+)["']/g)) {
values[match[1]] = match[2];
}

return { name, values };
}

function parseTsEnumFile(filePath: string): EnumDefinition | null {
const content = readFileSync(filePath, "utf8");
const enumMatch = content.match(/export\s+enum\s+(\w+)\s*\{([\s\S]*?)\}/);
if (!enumMatch) {
return null;
}

const [, name, body] = enumMatch;
const values: Record<string, string> = {};

for (const match of body.matchAll(/(\w+)\s*=\s*["']([^"']+)["']/g)) {
values[match[1]] = match[2];
}

return { name, values };
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
