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
}
