<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BreadcrumbList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;
use PHPUnit\Framework\TestCase;

final class BreadcrumbListTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new BreadcrumbList(itemListElement: [
			new ListItem(
				position: 1,
				name: 'Books',
				item: 'https://example.com/books',
			),
		]);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('BreadcrumbList', $obj->{'@type'});
		$this->assertCount(1, $obj->itemListElement);
		$this->assertEquals('ListItem', $obj->itemListElement[0]->{'@type'});
		$this->assertEquals(1, $obj->itemListElement[0]->position);
		$this->assertEquals('Books', $obj->itemListElement[0]->name);
		$this->assertEquals('https://example.com/books', $obj->itemListElement[0]->item);
	}

	public function testMultipleItems(): void {
		$schema = new BreadcrumbList(itemListElement: [
			new ListItem(position: 1, name: 'Books', item: 'https://example.com/books'),
			new ListItem(position: 2, name: 'Science Fiction', item: 'https://example.com/books/sciencefiction'),
			new ListItem(position: 3, name: 'Award Winners'),
		]);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(3, $obj->itemListElement);
		$this->assertEquals(1, $obj->itemListElement[0]->position);
		$this->assertEquals('Books', $obj->itemListElement[0]->name);
		$this->assertEquals(2, $obj->itemListElement[1]->position);
		$this->assertEquals('Science Fiction', $obj->itemListElement[1]->name);
		$this->assertEquals(3, $obj->itemListElement[2]->position);
		$this->assertEquals('Award Winners', $obj->itemListElement[2]->name);
	}

	public function testSingleItemWithoutItemUrlForCurrentPage(): void {
		$schema = new BreadcrumbList(itemListElement: [
			new ListItem(position: 1, name: 'Checkout'),
		]);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(1, $obj->itemListElement);
		$this->assertEquals('Checkout', $obj->itemListElement[0]->name);
		$this->assertFalse(property_exists($obj->itemListElement[0], 'item'));
	}

	public function testLongBreadcrumbChainWithOrderedPositions(): void {
		$schema = new BreadcrumbList(itemListElement: [
			new ListItem(position: 1, name: 'Home', item: 'https://example.com'),
			new ListItem(position: 2, name: 'Library', item: 'https://example.com/library'),
			new ListItem(position: 3, name: 'Books', item: 'https://example.com/library/books'),
			new ListItem(position: 4, name: 'Fiction', item: 'https://example.com/library/books/fiction'),
			new ListItem(position: 5, name: 'Classics'),
		]);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(5, $obj->itemListElement);
		$this->assertEquals(1, $obj->itemListElement[0]->position);
		$this->assertEquals(5, $obj->itemListElement[4]->position);
		$this->assertEquals('Classics', $obj->itemListElement[4]->name);
		$this->assertFalse(property_exists($obj->itemListElement[4], 'item'));
	}

	public function testListItemIncludesOptionalUrlField(): void {
		$schema = new BreadcrumbList(itemListElement: [
			new ListItem(
				position: 1,
				name: 'Products',
				item: 'https://example.com/products',
				url: 'https://example.com/list-item/products',
			),
			new ListItem(position: 2, name: 'Current Product'),
		]);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://example.com/list-item/products', $obj->itemListElement[0]->url);
		$this->assertFalse(property_exists($obj->itemListElement[1], 'item'));
	}
}
