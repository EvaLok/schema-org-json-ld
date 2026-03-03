<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Restaurant;
use PHPUnit\Framework\TestCase;

final class RestaurantTest extends TestCase {
	public function testMinimalOutput(): void {
		$restaurant = new Restaurant(
			name: 'Example Bistro',
			address: new PostalAddress(streetAddress: '123 Main Street'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $restaurant);
		$obj = json_decode($json);

		$this->assertEquals('Restaurant', $obj->{'@type'});
		$this->assertEquals('Example Bistro', $obj->name);
	}

	public function testFullOutput(): void {
		$restaurant = new Restaurant(
			name: 'Example Bistro',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			servesCuisine: 'Italian',
			menu: 'https://example.com/menu',
			acceptsReservations: true,
			priceRange: '$$',
			aggregateRating: new AggregateRating(
				ratingValue: 4.7,
				ratingCount: 145,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $restaurant);
		$obj = json_decode($json);

		$this->assertEquals('Restaurant', $obj->{'@type'});
		$this->assertEquals('Italian', $obj->servesCuisine);
		$this->assertEquals('https://example.com/menu', $obj->menu);
		$this->assertTrue($obj->acceptsReservations);
		$this->assertEquals('$$', $obj->priceRange);
		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
	}

	public function testTypeRemainsRestaurantWithInheritedLocalBusinessFields(): void {
		$restaurant = new Restaurant(
			name: 'Example Bistro',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			telephone: '+31-20-123-4567',
			url: 'https://example.com/restaurant',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $restaurant));

		$this->assertEquals('Restaurant', $obj->{'@type'});
		$this->assertEquals('+31-20-123-4567', $obj->telephone);
		$this->assertEquals('https://example.com/restaurant', $obj->url);
	}

	public function testAcceptsReservationsAsUrlString(): void {
		$restaurant = new Restaurant(
			name: 'Example Bistro',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			acceptsReservations: 'https://example.com/reservations',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $restaurant));

		$this->assertEquals('https://example.com/reservations', $obj->acceptsReservations);
	}

	public function testMenuAndCuisineSerializeTogether(): void {
		$restaurant = new Restaurant(
			name: 'Example Bistro',
			address: new PostalAddress(streetAddress: '123 Main Street'),
			menu: 'https://example.com/menu',
			servesCuisine: 'Italian',
		);
		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $restaurant));

		$this->assertEquals('https://example.com/menu', $obj->menu);
		$this->assertEquals('Italian', $obj->servesCuisine);
	}
}
