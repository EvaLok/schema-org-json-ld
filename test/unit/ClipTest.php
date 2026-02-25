<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Clip;
use PHPUnit\Framework\TestCase;

final class ClipTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Clip(
			name: 'Introduction',
			startOffset: 0,
			url: 'https://example.com/video?t=0',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Clip', $obj->{'@type'});
		$this->assertEquals('Introduction', $obj->name);
		$this->assertEquals(0, $obj->startOffset);
		$this->assertEquals('https://example.com/video?t=0', $obj->url);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Clip(
			name: 'Introduction',
			startOffset: 0,
			url: 'https://example.com/video?t=0',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('endOffset', $obj);
	}

	public function testFullOutput(): void {
		$schema = new Clip(
			name: 'Overview',
			startOffset: 30,
			url: 'https://example.com/video?t=30',
			endOffset: 89,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Overview', $obj->name);
		$this->assertEquals(30, $obj->startOffset);
		$this->assertEquals('https://example.com/video?t=30', $obj->url);
		$this->assertEquals(89, $obj->endOffset);
	}
}
