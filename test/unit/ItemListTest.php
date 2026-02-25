<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BreadcrumbList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Movie;
use PHPUnit\Framework\TestCase;

final class ItemListTest extends TestCase {
public function testSummaryPagePatternOutput(): void {
$schema = new ItemList(
itemListElement: [
new ListItem(position: 1, url: 'https://example.com/item1'),
new ListItem(position: 2, url: 'https://example.com/item2'),
],
);

$json = JsonLdGenerator::SchemaToJson(schema: $schema);
$obj = json_decode($json);

$this->assertEquals('https://schema.org/', $obj->{'@context'});
$this->assertEquals('ItemList', $obj->{'@type'});
$this->assertEquals('ListItem', $obj->itemListElement[0]->{'@type'});
$this->assertEquals(1, $obj->itemListElement[0]->position);
$this->assertEquals('https://example.com/item1', $obj->itemListElement[0]->url);
$this->assertFalse(property_exists($obj->itemListElement[0], 'name'));
}

public function testAllInOnePatternWithEmbeddedSchemaOutput(): void {
		$schema = new ItemList(
			itemListElement: [
				new ListItem(
					position: 1,
					item: new Movie(
						name: 'A Star Is Born',
						image: 'https://example.com/a-star-is-born.jpg',
					),
				),
			],
		);

$json = JsonLdGenerator::SchemaToJson(schema: $schema);
$obj = json_decode($json);

$this->assertEquals('ListItem', $obj->itemListElement[0]->{'@type'});
		$this->assertEquals(1, $obj->itemListElement[0]->position);
		$this->assertEquals('Movie', $obj->itemListElement[0]->item->{'@type'});
		$this->assertEquals('A Star Is Born', $obj->itemListElement[0]->item->name);
		$this->assertEquals('https://example.com/a-star-is-born.jpg', $obj->itemListElement[0]->item->image);
	}

public function testNumberOfItemsAndItemListOrderOutput(): void {
$schema = new ItemList(
itemListElement: [
new ListItem(position: 1, url: 'https://example.com/item1'),
],
itemListOrder: 'Ascending',
numberOfItems: 1,
);

$json = JsonLdGenerator::SchemaToJson(schema: $schema);
$obj = json_decode($json);

$this->assertEquals('Ascending', $obj->itemListOrder);
$this->assertEquals(1, $obj->numberOfItems);
}

public function testOptionalPropertiesOmittedWhenNull(): void {
$schema = new ItemList(
itemListElement: [
new ListItem(position: 1, url: 'https://example.com/item1'),
],
);

$json = JsonLdGenerator::SchemaToJson(schema: $schema);
$obj = json_decode($json);

$this->assertFalse(property_exists($obj, 'itemListOrder'));
$this->assertFalse(property_exists($obj, 'numberOfItems'));
}

public function testBreadcrumbListBackwardCompatibility(): void {
$schema = new BreadcrumbList(itemListElement: [
new ListItem(position: 1, name: 'Home', item: 'https://example.com/'),
new ListItem(position: 2, name: 'Category', item: 'https://example.com/category'),
]);

$json = JsonLdGenerator::SchemaToJson(schema: $schema);
$obj = json_decode($json);

$this->assertEquals('BreadcrumbList', $obj->{'@type'});
$this->assertEquals('Home', $obj->itemListElement[0]->name);
$this->assertEquals('https://example.com/', $obj->itemListElement[0]->item);
}
}
