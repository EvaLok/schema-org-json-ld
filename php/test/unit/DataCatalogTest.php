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
}
