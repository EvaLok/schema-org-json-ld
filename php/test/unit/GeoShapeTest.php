<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoShape;
use PHPUnit\Framework\TestCase;

final class GeoShapeTest extends TestCase {
	public function testWithBox(): void {
		$schema = new GeoShape(box: '-43.5 170.0 -35.0 178.6');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('GeoShape', $obj->{'@type'});
		$this->assertEquals('-43.5 170.0 -35.0 178.6', $obj->box);
	}

	public function testNullBoxOmitted(): void {
		$schema = new GeoShape();
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);
		$this->assertObjectNotHasProperty('box', $obj);
	}

	public function testMinimalOutputContainsOnlySchemaMetadata(): void {
		$schema = new GeoShape(box: null);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(['@context', '@type'], array_keys(get_object_vars($obj)));
	}

	public function testValidBoxStringIsSerialized(): void {
		$schema = new GeoShape(box: '37.42242 -122.08585 37.42242 -122.08585');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('37.42242 -122.08585 37.42242 -122.08585', $obj->box);
	}

	public function testEmptyStringBoxIsSerialized(): void {
		$schema = new GeoShape(box: '');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('', $obj->box);
	}
}
