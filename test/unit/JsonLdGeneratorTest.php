<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use PHPUnit\Framework\TestCase;

final class JsonLdGeneratorTest extends TestCase {
	public function testShouldGenerateValidJsonLd() {
		$product = new Product(
			name: "Executive Anvil",
			image: [
				"https://example.com/photos/1x1/photo.jpg",
				"https://example.com/photos/4x3/photo.jpg",
				"https://example.com/photos/16x9/photo.jpg"
			],
			description: "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
			sku: "0446310786",
			offers: [
				new Offer(
					url: "https://example.com/anvil",
					priceCurrency: "USD",
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock
				)
			]
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $product
		);

		$this->assertIsString($json);

		$output_json_obj = json_decode($json);
		$comparison_obj = json_decode(file_get_contents(__DIR__ . '/../samples/Product.json'));

		$this->assertEquals($comparison_obj, $output_json_obj, "resultant json_decode objects should be equal");

	}
}
