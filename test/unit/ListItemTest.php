<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Movie;
use PHPUnit\Framework\TestCase;

final class ListItemTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new ListItem(position: 1, name: 'Home');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ListItem', $obj->{'@type'});
		$this->assertEquals(1, $obj->position);
		$this->assertEquals('Home', $obj->name);
	}

	public function testOutputWithItemStringUrl(): void {
		$schema = new ListItem(position: 2, item: 'https://example.com/category');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://example.com/category', $obj->item);
	}

	public function testOutputWithItemTypedSchema(): void {
		$schema = new ListItem(
			position: 3,
			item: new Movie(
				name: 'A Star Is Born',
				image: 'https://example.com/a-star-is-born.jpg',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Movie', $obj->item->{'@type'});
		$this->assertEquals('A Star Is Born', $obj->item->name);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new ListItem(position: 1);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'item'));
		$this->assertFalse(property_exists($obj, 'url'));
	}
}
