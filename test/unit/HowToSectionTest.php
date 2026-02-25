<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\HowToSection;
use EvaLok\SchemaOrgJsonLd\v1\Schema\HowToStep;
use PHPUnit\Framework\TestCase;

final class HowToSectionTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new HowToSection(
			name: 'Prepare the filling',
			itemListElement: [
				new HowToStep(text: 'Dice the onions.'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('HowToSection', $obj->{'@type'});
		$this->assertEquals('Prepare the filling', $obj->name);
		$this->assertEquals('HowToStep', $obj->itemListElement[0]->{'@type'});
		$this->assertEquals('Dice the onions.', $obj->itemListElement[0]->text);
	}

	public function testMultipleStepsInItemListElement(): void {
		$schema = new HowToSection(
			name: 'Assemble the pie',
			itemListElement: [
				new HowToStep(text: 'Roll out the dough.'),
				new HowToStep(text: 'Fill with the prepared mixture.'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('HowToSection', $obj->{'@type'});
		$this->assertCount(2, $obj->itemListElement);
		$this->assertEquals('HowToStep', $obj->itemListElement[0]->{'@type'});
		$this->assertEquals('Roll out the dough.', $obj->itemListElement[0]->text);
		$this->assertEquals('HowToStep', $obj->itemListElement[1]->{'@type'});
		$this->assertEquals('Fill with the prepared mixture.', $obj->itemListElement[1]->text);
	}
}
