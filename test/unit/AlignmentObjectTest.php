<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AlignmentObject;
use PHPUnit\Framework\TestCase;

final class AlignmentObjectTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new AlignmentObject(
			alignmentType: 'educationalSubject',
			targetName: 'Mathematics',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('AlignmentObject', $obj->{'@type'});
		$this->assertEquals('educationalSubject', $obj->alignmentType);
		$this->assertEquals('Mathematics', $obj->targetName);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new AlignmentObject(
			alignmentType: 'educationalSubject',
			targetName: 'Mathematics',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'educationalFramework'));
		$this->assertFalse(property_exists($obj, 'targetUrl'));
	}

	public function testFullOutput(): void {
		$schema = new AlignmentObject(
			alignmentType: 'educationalSubject',
			targetName: 'Mathematics',
			educationalFramework: 'Example Curriculum',
			targetUrl: 'https://example.org/framework/math',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Example Curriculum', $obj->educationalFramework);
		$this->assertEquals('https://example.org/framework/math', $obj->targetUrl);
	}
}
