<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Thing;
use PHPUnit\Framework\TestCase;

final class ThingTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Thing(name: 'Executive Anvil');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Thing', $obj->{'@type'});
		$this->assertEquals('Executive Anvil', $obj->name);
	}

	public function testEmptyStringNameIsSerialized(): void {
		$schema = new Thing(name: '');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('', $obj->name);
	}

	public function testOnlyContextTypeAndNameAppear(): void {
		$schema = new Thing(name: 'Widget');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(['@context', '@type', 'name'], array_keys(get_object_vars($obj)));
	}

	public function testExactNameValueRoundTrips(): void {
		$schema = new Thing(name: 'Thing 42 / sample');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('Thing 42 / sample', $obj->name);
	}
}
