<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AdministrativeArea;
use PHPUnit\Framework\TestCase;

final class AdministrativeAreaTest extends TestCase {
	public function testOutput(): void {
		$schema = new AdministrativeArea(name: 'California');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('AdministrativeArea', $obj->{'@type'});
		$this->assertEquals('California', $obj->name);
	}

	public function testEmptyStringNameIsSerialized(): void {
		$schema = new AdministrativeArea(name: '');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('', $obj->name);
	}

	public function testOnlyContextTypeAndNameAppear(): void {
		$schema = new AdministrativeArea(name: 'Ontario');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(['@context', '@type', 'name'], array_keys(get_object_vars($obj)));
	}

	public function testExactNameValueRoundTrips(): void {
		$schema = new AdministrativeArea(name: 'Île-de-France');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('Île-de-France', $obj->name);
	}
}
