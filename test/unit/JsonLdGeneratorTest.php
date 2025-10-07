<?php

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BreadcrumbList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferShippingDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingDeliveryTime;
use PHPUnit\Framework\TestCase;

final class JsonLdGeneratorTest extends TestCase {
	public function testShouldGenerateValidJsonLdWithMinimalData() {
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
		$comparison_obj = json_decode(file_get_contents(__DIR__ . '/../samples/Product.minimal.json'));

		$this->assertEquals($comparison_obj, $output_json_obj, "resultant json_decode objects should be equal");

	}

	public function testShouldGenerateValidJsonLdWithFullData() {
		$product = new Product(
			name: "Executive Anvil",
			image: [
				"https://example.com/photos/1x1/photo.jpg",
				"https://example.com/photos/4x3/photo.jpg",
				"https://example.com/photos/16x9/photo.jpg"
			],
			description: "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
			sku: "0446310786",
			brand: new Brand(
				name: "ACME (tm)",
			),
			mpn: "ACME0444246625",
			weight: new QuantitativeValue(
				value: 55.67,
				unitCode: "LBR"
			),
			offers: [
				new Offer(
					url: "https://example.com/anvil",
					priceCurrency: "USD",
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
					shippingDetails: [
						new OfferShippingDetails(
							shippingDestination: new DefinedRegion(
								addressCountry: "US",
								addressRegion: [ "CA", "NV", "AZ" ]
							),
							shippingRate: new MonetaryAmount(
								value: 3.49,
								currency: "USD",
							),
							deliveryTime: new ShippingDeliveryTime(
								handlingTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 0,
									maxValue: 1
								),
								transitTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 1,
									maxValue: 5
								)
							)
						),
						new OfferShippingDetails(
							shippingDestination: new DefinedRegion(
								addressCountry: "US",
								addressRegion: [ "HI" ]
							),
							shippingRate: new MonetaryAmount(
								value: 77.49,
								currency: "USD",
							),
							deliveryTime: new ShippingDeliveryTime(
								handlingTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 0,
									maxValue: 1
								),
								transitTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 4,
									maxValue: 10
								)
							)
						),
						new OfferShippingDetails(
							shippingDestination: new DefinedRegion(
								addressCountry: "US",
								addressRegion: [ "AK" ]
							),
							doesNotShip: true,
						),
					]
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

	public function testShouldGenerateValidBreadcrumbList() {
		$breadcrumbList = new BreadcrumbList(
			itemListElement: [
				new ListItem(
					position: 1,
					name: "Books",
					item: "https://example.com/books"
				),
				new ListItem(
					position: 2,
					name: "Science Fiction",
					item: "https://example.com/books/sciencefiction"
				),
				new ListItem(
					position: 3,
					name: "Award Winners"
				)
			]
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $breadcrumbList
		);

		$this->assertIsString($json);

		$output_json_obj = json_decode($json);
		$comparison_obj = json_decode(file_get_contents(__DIR__ . '/../samples/BreadcrumbList.json'));

		$this->assertEquals($comparison_obj, $output_json_obj, "resultant json_decode objects should be equal");
	}
}
