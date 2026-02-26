<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\VirtualLocation;
use PHPUnit\Framework\TestCase;

final class VirtualLocationTest extends TestCase {
	public function testMinimalOutput(): void {
		$virtualLocation = new VirtualLocation(url: 'https://example.com/join');
		$json = JsonLdGenerator::SchemaToJson(schema: $virtualLocation);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('VirtualLocation', $obj->{'@type'});
		$this->assertEquals('https://example.com/join', $obj->url);
		$this->assertFalse(property_exists($obj, 'name'));
	}

	public function testFullOutput(): void {
		$virtualLocation = new VirtualLocation(
			url: 'https://example.com/join',
			name: 'Virtual Stage',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $virtualLocation);
		$obj = json_decode($json);

		$this->assertEquals('VirtualLocation', $obj->{'@type'});
		$this->assertEquals('https://example.com/join', $obj->url);
		$this->assertEquals('Virtual Stage', $obj->name);
	}
}
