<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SolveMathAction;
use PHPUnit\Framework\TestCase;

final class SolveMathActionTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('SolveMathAction', $obj->{'@type'});
		$this->assertEquals('https://mathdomain.com/solve?q={math_expression_string}', $obj->target);
		// Verify property is remapped to hyphenated name
		$this->assertEquals('required name=math_expression_string', $obj->{'mathExpression-input'});
		// Verify PHP property name doesn't exist in output
		$this->assertFalse(property_exists($obj, 'mathExpressionInput'));
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'eduQuestionType'));
	}

	public function testEduQuestionTypeAsString(): void {
		$schema = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: 'Polynomial Equation',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Polynomial Equation', $obj->eduQuestionType);
		$this->assertIsString($obj->eduQuestionType);
	}

	public function testEduQuestionTypeAsArray(): void {
		$schema = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: ['Polynomial Equation', 'Quadratic Equation'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertIsArray($obj->eduQuestionType);
		$this->assertCount(2, $obj->eduQuestionType);
		$this->assertEquals('Polynomial Equation', $obj->eduQuestionType[0]);
		$this->assertEquals('Quadratic Equation', $obj->eduQuestionType[1]);
	}

	public function testFullOutput(): void {
		$schema = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: ['Polynomial Equation', 'Linear Equation'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('SolveMathAction', $obj->{'@type'});
		$this->assertEquals('https://mathdomain.com/solve?q={math_expression_string}', $obj->target);
		$this->assertEquals('required name=math_expression_string', $obj->{'mathExpression-input'});
		$this->assertIsArray($obj->eduQuestionType);
		$this->assertCount(2, $obj->eduQuestionType);
	}
}
