<?php

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
}
