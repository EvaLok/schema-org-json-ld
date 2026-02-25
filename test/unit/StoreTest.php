<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Store;
use PHPUnit\Framework\TestCase;

final class StoreTest extends TestCase {
	public function testMinimalOutput(): void {
		$store = new Store(
			name: 'Example Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $store);
		$obj = json_decode($json);

		$this->assertEquals('Store', $obj->{'@type'});
		$this->assertEquals('Example Store', $obj->name);
	}

	public function testInheritedProperties(): void {
		$store = new Store(
			name: 'Example Store',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			telephone: '+31-20-123-4567',
			url: 'https://example.com',
			priceRange: '$$',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $store);
		$obj = json_decode($json);

		$this->assertEquals('Store', $obj->{'@type'});
		$this->assertEquals('+31-20-123-4567', $obj->telephone);
		$this->assertEquals('https://example.com', $obj->url);
		$this->assertEquals('$$', $obj->priceRange);
	}
}
