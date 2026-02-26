<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\HowToStep;
use PHPUnit\Framework\TestCase;

final class HowToStepTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new HowToStep(text: 'Mix all ingredients.');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('HowToStep', $obj->{'@type'});
		$this->assertEquals('Mix all ingredients.', $obj->text);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new HowToStep(text: 'Mix all ingredients.');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'url'));
		$this->assertFalse(property_exists($obj, 'image'));
	}

	public function testFullOutput(): void {
		$schema = new HowToStep(
			text: 'Bake for 20 minutes.',
			name: 'Bake',
			url: 'https://example.com/recipe#step2',
			image: 'https://example.com/step2.jpg',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Bake for 20 minutes.', $obj->text);
		$this->assertEquals('Bake', $obj->name);
		$this->assertEquals('https://example.com/recipe#step2', $obj->url);
		$this->assertEquals('https://example.com/step2.jpg', $obj->image);
	}
}
