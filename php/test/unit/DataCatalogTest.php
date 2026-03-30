<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DataCatalog;
use PHPUnit\Framework\TestCase;

final class DataCatalogTest extends TestCase {
	public function testBasicConstruction(): void {
		$schema = new DataCatalog(name: 'My Data Catalog');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('DataCatalog', $obj->{'@type'});
		$this->assertEquals('My Data Catalog', $obj->name);
	}

	public function testEmptyStringNameIsSerialized(): void {
		$schema = new DataCatalog(name: '');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('', $obj->name);
	}

	public function testOnlyContextTypeAndNameAreSerialized(): void {
		$schema = new DataCatalog(name: 'Open Data');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(['@context', '@type', 'name'], array_keys(get_object_vars($obj)));
	}

	public function testNameValueIsPreservedExactly(): void {
		$schema = new DataCatalog(name: 'Catalog 2026');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('Catalog 2026', $obj->name);
	}
}
