<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use PHPUnit\Framework\TestCase;

final class DefinedRegionTest extends TestCase {
	public function testOutput(): void {
		$schema = new DefinedRegion(
			addressCountry: 'US',
			addressRegion: ['CA', 'NV'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('DefinedRegion', $obj->{'@type'});
		$this->assertEquals('US', $obj->addressCountry);
		$this->assertEquals('CA', $obj->addressRegion[0]);
		$this->assertEquals('NV', $obj->addressRegion[1]);
	}
}
