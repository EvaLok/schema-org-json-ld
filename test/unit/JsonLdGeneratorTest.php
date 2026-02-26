<?php

declare(strict_types=1);

use EvaLok\SchemaOrgJsonLd\v1\Enum\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Enum\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Article;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BreadcrumbList;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ListItem;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnPolicy;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnPolicySeasonalOverride;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferShippingDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingDeliveryTime;
use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;
use PHPUnit\Framework\TestCase;

final class JsonLdGeneratorTest extends TestCase {
	public function testShouldGenerateValidJsonLdWithMinimalData() {
		$product = new Product(
			name: "Executive Anvil",
			image: [
				"https://example.com/photos/1x1/photo.jpg",
				"https://example.com/photos/4x3/photo.jpg",
				"https://example.com/photos/16x9/photo.jpg",
			],
			description: "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
			sku: "0446310786",
			offers: [
				new Offer(
					url: "https://example.com/anvil",
					priceCurrency: "USD",
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $product,
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
				"https://example.com/photos/16x9/photo.jpg",
			],
			description: "Sleeker than ACME's Classic Anvil, the Executive Anvil is perfect for the business traveler looking for something to drop from a height.",
			sku: "0446310786",
			brand: new Brand(
				name: "ACME (tm)",
			),
			mpn: "ACME0444246625",
			weight: new QuantitativeValue(
				value: 55.67,
				unitCode: "LBR",
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
								addressRegion: [ "CA", "NV", "AZ" ],
							),
							shippingRate: new MonetaryAmount(
								value: 3.49,
								currency: "USD",
							),
							deliveryTime: new ShippingDeliveryTime(
								handlingTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 0,
									maxValue: 1,
								),
								transitTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 1,
									maxValue: 5,
								),
							),
						),
						new OfferShippingDetails(
							shippingDestination: new DefinedRegion(
								addressCountry: "US",
								addressRegion: [ "HI" ],
							),
							shippingRate: new MonetaryAmount(
								value: 77.49,
								currency: "USD",
							),
							deliveryTime: new ShippingDeliveryTime(
								handlingTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 0,
									maxValue: 1,
								),
								transitTime: new QuantitativeValue(
									unitCode: "DAY",
									minValue: 4,
									maxValue: 10,
								),
							),
						),
						new OfferShippingDetails(
							shippingDestination: new DefinedRegion(
								addressCountry: "US",
								addressRegion: [ "AK" ],
							),
							doesNotShip: true,
						),
					],
				),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $product,
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
					item: "https://example.com/books",
				),
				new ListItem(
					position: 2,
					name: "Science Fiction",
					item: "https://example.com/books/sciencefiction",
				),
				new ListItem(
					position: 3,
					name: "Award Winners",
				),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $breadcrumbList,
		);

		$this->assertIsString($json);

		$output_json_obj = json_decode($json);
		$comparison_obj = json_decode(file_get_contents(__DIR__ . '/../samples/BreadcrumbList.json'));

		$this->assertEquals($comparison_obj, $output_json_obj, "resultant json_decode objects should be equal");
	}

	public function testShouldSkipEmptyArrayProperties() {
		$breadcrumbList = new BreadcrumbList(
			itemListElement: [],
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $breadcrumbList,
		);

		$this->assertIsString($json);

		$output_json_obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $output_json_obj->{'@context'});
		$this->assertEquals('BreadcrumbList', $output_json_obj->{'@type'});
		$this->assertObjectNotHasProperty('itemListElement', $output_json_obj);
	}

	public function testShouldHandleNonEmptyTypedSchemaArrays() {
		$breadcrumbList = new BreadcrumbList(
			itemListElement: [
				new ListItem(
					position: 1,
					name: "Books",
					item: "https://example.com/books",
				),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $breadcrumbList,
		);

		$this->assertIsString($json);

		$output_json_obj = json_decode($json);
		$this->assertCount(1, $output_json_obj->itemListElement);
		$this->assertEquals(1, $output_json_obj->itemListElement[0]->position);
		$this->assertEquals('Books', $output_json_obj->itemListElement[0]->name);
		$this->assertEquals('https://example.com/books', $output_json_obj->itemListElement[0]->item);
	}

	public function testShouldHandleNonEmptyStringArrays() {
		$product = new Product(
			name: "Executive Anvil",
			image: [
				"https://example.com/photos/1x1/photo.jpg",
				"https://example.com/photos/4x3/photo.jpg",
			],
			description: "An anvil",
			sku: "0446310786",
			offers: [
				new Offer(
					url: "https://example.com/anvil",
					priceCurrency: "USD",
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(
			schema: $product,
		);

		$this->assertIsString($json);

		$output_json_obj = json_decode($json);
		$this->assertCount(2, $output_json_obj->image);
		$this->assertEquals('https://example.com/photos/1x1/photo.jpg', $output_json_obj->image[0]);
		$this->assertEquals('https://example.com/photos/4x3/photo.jpg', $output_json_obj->image[1]);
	}

	public function testShouldExcludeEmptyArrayPropertyOnProduct(): void {
		$product = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'An anvil',
			sku: '0446310786',
			offers: [],
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $product);
		$obj = json_decode($json);

		$this->assertEquals('Product', $obj->{'@type'});
		$this->assertObjectNotHasProperty('offers', $obj);
	}

	public function testShouldSerializeSpecialCharactersInStrings(): void {
		$product = new Product(
			name: 'Café "Élite" & Friends <Best>',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: "AT&T &amp; Unicode snowman ☃ and emoji 😊",
			sku: 'sku-"001"&x',
			offers: [
				new Offer(
					url: 'https://example.com/anvil?name="elite"&q=1',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $product);
		$obj = json_decode($json);

		$this->assertEquals('Café "Élite" & Friends <Best>', $obj->name);
		$this->assertEquals('AT&T &amp; Unicode snowman ☃ and emoji 😊', $obj->description);
		$this->assertEquals('sku-"001"&x', $obj->sku);
		$this->assertEquals('https://example.com/anvil?name="elite"&q=1', $obj->offers[0]->url);
	}

	public function testShouldSerializeDeeplyNestedObjectGraphWithMerchantReturnPolicySeasonalOverride(): void {
		$product = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'An anvil',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);
		$organization = new Organization(
			name: 'ACME Corp',
			hasMerchantReturnPolicy: [
				new MerchantReturnPolicy(
					applicableCountry: ['US', 'CA'],
					returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
					merchantReturnDays: 30,
					returnPolicySeasonalOverride: [
						new MerchantReturnPolicySeasonalOverride(
							startDate: '2026-11-01',
							endDate: '2026-12-31',
							returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
							merchantReturnDays: 60,
						),
					],
				),
			],
		);

		$json = JsonLdGenerator::SchemasToJson($product, $organization);
		$obj = json_decode($json);

		$this->assertEquals('Product', $obj->{'@graph'}[0]->{'@type'});
		$this->assertEquals('Offer', $obj->{'@graph'}[0]->offers[0]->{'@type'});
		$this->assertEquals('Organization', $obj->{'@graph'}[1]->{'@type'});
		$this->assertEquals('MerchantReturnPolicy', $obj->{'@graph'}[1]->hasMerchantReturnPolicy[0]->{'@type'});
		$this->assertEquals(
			'MerchantReturnPolicySeasonalOverride',
			$obj->{'@graph'}[1]->hasMerchantReturnPolicy[0]->returnPolicySeasonalOverride[0]->{'@type'},
		);
	}

	public function testShouldSerializeSingleAndMultipleOfferArrays(): void {
		$singleOfferProduct = new Product(
			name: 'Single Offer Product',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Single offer',
			sku: 'single-offer',
			offers: [
				new Offer(
					url: 'https://example.com/single',
					priceCurrency: 'USD',
					price: 10.0,
					availability: ItemAvailability::InStock,
				),
			],
		);
		$multiOfferProduct = new Product(
			name: 'Multi Offer Product',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Multi offer',
			sku: 'multi-offer',
			offers: [
				new Offer(
					url: 'https://example.com/one',
					priceCurrency: 'USD',
					price: 10.0,
					availability: ItemAvailability::InStock,
				),
				new Offer(
					url: 'https://example.com/two',
					priceCurrency: 'USD',
					price: 20.0,
					availability: ItemAvailability::OutOfStock,
				),
			],
		);

		$singleObj = json_decode(JsonLdGenerator::SchemaToJson(schema: $singleOfferProduct));
		$multiObj = json_decode(JsonLdGenerator::SchemaToJson(schema: $multiOfferProduct));

		$this->assertCount(1, $singleObj->offers);
		$this->assertEquals('https://example.com/single', $singleObj->offers[0]->url);
		$this->assertCount(2, $multiObj->offers);
		$this->assertEquals('https://example.com/one', $multiObj->offers[0]->url);
		$this->assertEquals('https://example.com/two', $multiObj->offers[1]->url);
	}

	public function testShouldSerializeMixedScalarArrays(): void {
		$region = new DefinedRegion(
			addressCountry: 'US',
			addressRegion: ['CA', 90210, true, ItemAvailability::InStock],
		);

		$obj = json_decode(JsonLdGenerator::SchemaToJson(schema: $region));

		$this->assertCount(4, $obj->addressRegion);
		$this->assertEquals('CA', $obj->addressRegion[0]);
		$this->assertEquals(90210, $obj->addressRegion[1]);
		$this->assertTrue($obj->addressRegion[2]);
		$this->assertEquals(ItemAvailability::InStock->value, $obj->addressRegion[3]);
	}

	public function testShouldHandleArrayTypeCorrectly() {
		// Create a temporary test schema with array @type
		$testSchemaClass = new class ('') extends TypedSchema {
			public const A_SCHEMA_TYPE = ['TestType1', 'TestType2'];

			public function __construct(
				public string $name,
			) {}
		};

		$schema = new ($testSchemaClass::class)(name: 'Test Schema');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertIsArray($obj->{'@type'});
		$this->assertCount(2, $obj->{'@type'});
		$this->assertEquals('TestType1', $obj->{'@type'}[0]);
		$this->assertEquals('TestType2', $obj->{'@type'}[1]);
		$this->assertEquals('Test Schema', $obj->name);
	}

	public function testShouldApplyPropertyMapWhenPresent() {
		// Create a temporary test schema with PROPERTY_MAP
		$testSchemaClass = new class ('', '', '') extends TypedSchema {
			public const A_SCHEMA_TYPE = 'TestTypeWithMap';
			public const PROPERTY_MAP = [
				'phpPropertyName' => 'json-property-name',
				'anotherProperty' => 'another-json-property',
			];

			public function __construct(
				public string $phpPropertyName,
				public string $anotherProperty,
				public string $regularProperty,
			) {}
		};

		$schema = new ($testSchemaClass::class)(
			phpPropertyName: 'value1',
			anotherProperty: 'value2',
			regularProperty: 'value3',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('TestTypeWithMap', $obj->{'@type'});
		// Mapped properties should use the JSON-LD names
		$this->assertEquals('value1', $obj->{'json-property-name'});
		$this->assertEquals('value2', $obj->{'another-json-property'});
		// Regular property should remain unchanged
		$this->assertEquals('value3', $obj->regularProperty);
		// Original PHP property names should not exist in output
		$this->assertFalse(property_exists($obj, 'phpPropertyName'));
		$this->assertFalse(property_exists($obj, 'anotherProperty'));
	}

	public function testShouldNotBreakSchemasWithoutPropertyMap() {
		// Ensure existing schemas without PROPERTY_MAP still work correctly (regression test)
		$brand = new Brand(name: 'Test Brand', description: 'Test Description');
		$json = JsonLdGenerator::SchemaToJson(schema: $brand);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Brand', $obj->{'@type'});
		$this->assertEquals('Test Brand', $obj->name);
		$this->assertEquals('Test Description', $obj->description);
	}

	public function testSchemasToJsonWithTwoSchemas(): void {
		$article = new Article(headline: 'Test article');
		$breadcrumbList = new BreadcrumbList(
			itemListElement: [
				new ListItem(position: 1, name: 'Home', item: 'https://example.com'),
			],
		);

		$json = JsonLdGenerator::SchemasToJson($article, $breadcrumbList);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertIsArray($obj->{'@graph'});
		$this->assertCount(2, $obj->{'@graph'});
		$this->assertEquals('Article', $obj->{'@graph'}[0]->{'@type'});
		$this->assertEquals('BreadcrumbList', $obj->{'@graph'}[1]->{'@type'});
		$this->assertObjectNotHasProperty('@context', $obj->{'@graph'}[0]);
		$this->assertObjectNotHasProperty('@context', $obj->{'@graph'}[1]);
	}

	public function testSchemasToJsonWithSingleSchema(): void {
		$article = new Article(headline: 'Single schema');

		$json = JsonLdGenerator::SchemasToJson($article);
		$obj = json_decode($json);

		$this->assertIsArray($obj->{'@graph'});
		$this->assertCount(1, $obj->{'@graph'});
		$this->assertEquals('Article', $obj->{'@graph'}[0]->{'@type'});
	}

	public function testSchemasToJsonWithThreeSchemas(): void {
		$article = new Article(headline: 'Three schemas');
		$breadcrumbList = new BreadcrumbList(
			itemListElement: [
				new ListItem(position: 1, name: 'Home', item: 'https://example.com'),
			],
		);
		$organization = new Organization(name: 'Example Org');

		$json = JsonLdGenerator::SchemasToJson($article, $breadcrumbList, $organization);
		$obj = json_decode($json);

		$this->assertCount(3, $obj->{'@graph'});
		$this->assertEquals('Article', $obj->{'@graph'}[0]->{'@type'});
		$this->assertEquals('BreadcrumbList', $obj->{'@graph'}[1]->{'@type'});
		$this->assertEquals('Organization', $obj->{'@graph'}[2]->{'@type'});
	}

	public function testSchemasToObjectReturnsArray(): void {
		$article = new Article(headline: 'Array output');
		$organization = new Organization(name: 'Example Org');

		$obj = JsonLdGenerator::SchemasToObject($article, $organization);

		$this->assertIsArray($obj);
		$this->assertEquals('https://schema.org/', $obj['@context']);
		$this->assertIsArray($obj['@graph']);
		$this->assertCount(2, $obj['@graph']);
		$this->assertEquals('Article', $obj['@graph'][0]['@type']);
		$this->assertEquals('Organization', $obj['@graph'][1]['@type']);
	}

	public function testGraphElementsHaveNoContext(): void {
		$article = new Article(headline: 'No context in graph items');
		$organization = new Organization(name: 'Example Org');

		$obj = JsonLdGenerator::SchemasToObject($article, $organization);

		foreach ($obj['@graph'] as $graphElement) {
			$this->assertArrayNotHasKey('@context', $graphElement);
		}
	}

	public function testSchemaToJsonThrowsRuntimeExceptionWhenJsonEncodeFails(): void {
		$schema = new class ("\xB1\x31") extends TypedSchema {
			public const A_SCHEMA_TYPE = 'InvalidUtf8Schema';

			public function __construct(
				public string $name,
			) {}
		};

		$this->expectException(\RuntimeException::class);
		$this->expectExceptionMessage('Malformed UTF-8 characters');

		JsonLdGenerator::SchemaToJson($schema);
	}

	public function testSchemasToJsonThrowsRuntimeExceptionWhenJsonEncodeFails(): void {
		$schema = new class ("\xB1\x31") extends TypedSchema {
			public const A_SCHEMA_TYPE = 'InvalidUtf8Schema';

			public function __construct(
				public string $name,
			) {}
		};

		$this->expectException(\RuntimeException::class);
		$this->expectExceptionMessage('Malformed UTF-8 characters');

		JsonLdGenerator::SchemasToJson($schema);
	}
}
