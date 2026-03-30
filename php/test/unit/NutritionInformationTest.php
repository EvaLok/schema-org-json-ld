<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\NutritionInformation;
use PHPUnit\Framework\TestCase;

final class NutritionInformationTest extends TestCase {
	public function testMinimalOutputWithAllOptionalFields(): void {
		$schema = new NutritionInformation();
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('NutritionInformation', $obj->{'@type'});
		$this->assertFalse(property_exists($obj, 'calories'));
	}

	public function testFullOutput(): void {
		$schema = new NutritionInformation(
			calories: '240 calories',
			fatContent: '9 grams',
			saturatedFatContent: '1.5 grams',
			cholesterolContent: '0 milligrams',
			sodiumContent: '100 milligrams',
			carbohydrateContent: '36 grams',
			fiberContent: '2 grams',
			sugarContent: '13 grams',
			proteinContent: '4 grams',
			servingSize: '1 cup',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('240 calories', $obj->calories);
		$this->assertEquals('9 grams', $obj->fatContent);
		$this->assertEquals('1.5 grams', $obj->saturatedFatContent);
		$this->assertEquals('0 milligrams', $obj->cholesterolContent);
		$this->assertEquals('100 milligrams', $obj->sodiumContent);
		$this->assertEquals('36 grams', $obj->carbohydrateContent);
		$this->assertEquals('2 grams', $obj->fiberContent);
		$this->assertEquals('13 grams', $obj->sugarContent);
		$this->assertEquals('4 grams', $obj->proteinContent);
		$this->assertEquals('1 cup', $obj->servingSize);
	}

	public function testSingleFieldOutputOmitsRemainingNullFields(): void {
		$schema = new NutritionInformation(
			proteinContent: '12 grams',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('12 grams', $obj->proteinContent);
		$this->assertFalse(property_exists($obj, 'calories'));
		$this->assertFalse(property_exists($obj, 'fatContent'));
		$this->assertFalse(property_exists($obj, 'servingSize'));
	}

	public function testEmptyStringFieldIsSerialized(): void {
		$schema = new NutritionInformation(
			calories: '',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame('', $obj->calories);
	}

	public function testNullFieldsAreOmittedFromOutput(): void {
		$schema = new NutritionInformation(
			calories: null,
			fatContent: null,
			saturatedFatContent: null,
			cholesterolContent: null,
			sodiumContent: null,
			carbohydrateContent: null,
			fiberContent: null,
			sugarContent: null,
			proteinContent: null,
			servingSize: null,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'calories'));
		$this->assertFalse(property_exists($obj, 'fatContent'));
		$this->assertFalse(property_exists($obj, 'saturatedFatContent'));
		$this->assertFalse(property_exists($obj, 'cholesterolContent'));
		$this->assertFalse(property_exists($obj, 'sodiumContent'));
		$this->assertFalse(property_exists($obj, 'carbohydrateContent'));
		$this->assertFalse(property_exists($obj, 'fiberContent'));
		$this->assertFalse(property_exists($obj, 'sugarContent'));
		$this->assertFalse(property_exists($obj, 'proteinContent'));
		$this->assertFalse(property_exists($obj, 'servingSize'));
	}
}
