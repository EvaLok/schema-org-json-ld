<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Clip;
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
		$this->assertFalse(property_exists($obj, 'video'));
		$this->assertFalse(property_exists($obj, 'itemListElement'));
	}

	public function testVideoSerializesAsNestedClip(): void {
		$video = new Clip(
			name: 'Step 2 clip',
			startOffset: 10,
			url: 'https://example.com/recipe-video#clip-2',
			endOffset: 22,
		);
		$schema = new HowToStep(
			text: 'Bake for 20 minutes.',
			video: $video,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Clip', $obj->video->{'@type'});
		$this->assertEquals('Step 2 clip', $obj->video->name);
		$this->assertEquals(10, $obj->video->startOffset);
		$this->assertEquals('https://example.com/recipe-video#clip-2', $obj->video->url);
		$this->assertEquals(22, $obj->video->endOffset);
	}

	public function testItemListElementSerializesAsStringArray(): void {
		$schema = new HowToStep(
			text: 'Bake for 20 minutes.',
			itemListElement: ['Preheat oven', 'Place tray inside'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(['Preheat oven', 'Place tray inside'], $obj->itemListElement);
	}

	public function testFullOutput(): void {
		$video = new Clip(
			name: 'Step 2 clip',
			startOffset: 10,
			url: 'https://example.com/recipe-video#clip-2',
		);
		$schema = new HowToStep(
			text: 'Bake for 20 minutes.',
			name: 'Bake',
			url: 'https://example.com/recipe#step2',
			image: 'https://example.com/step2.jpg',
			video: $video,
			itemListElement: ['Preheat oven', 'Place tray inside'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Bake for 20 minutes.', $obj->text);
		$this->assertEquals('Bake', $obj->name);
		$this->assertEquals('https://example.com/recipe#step2', $obj->url);
		$this->assertEquals('https://example.com/step2.jpg', $obj->image);
		$this->assertEquals('Clip', $obj->video->{'@type'});
		$this->assertEquals('Step 2 clip', $obj->video->name);
		$this->assertEquals(['Preheat oven', 'Place tray inside'], $obj->itemListElement);
	}
}
