<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MathSolver;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SolveMathAction;
use PHPUnit\Framework\TestCase;

final class MathSolverTest extends TestCase {
	public function testMinimalOutput(): void {
		$action = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
		);

		$schema = new MathSolver(
			url: 'https://www.mathdomain.com/',
			usageInfo: 'https://www.mathdomain.com/privacy',
			potentialAction: $action,
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		// Verify @type is an array
		$this->assertIsArray($obj->{'@type'});
		$this->assertCount(2, $obj->{'@type'});
		$this->assertEquals('MathSolver', $obj->{'@type'}[0]);
		$this->assertEquals('LearningResource', $obj->{'@type'}[1]);
		$this->assertEquals('https://www.mathdomain.com/', $obj->url);
		$this->assertEquals('https://www.mathdomain.com/privacy', $obj->usageInfo);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$action = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
		);

		$schema = new MathSolver(
			url: 'https://www.mathdomain.com/',
			usageInfo: 'https://www.mathdomain.com/privacy',
			potentialAction: $action,
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'inLanguage'));
		$this->assertFalse(property_exists($obj, 'learningResourceType'));
		$this->assertFalse(property_exists($obj, 'assesses'));
	}

	public function testSinglePotentialAction(): void {
		$action = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: 'Polynomial Equation',
		);

		$schema = new MathSolver(
			url: 'https://www.mathdomain.com/',
			usageInfo: 'https://www.mathdomain.com/privacy',
			potentialAction: $action,
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		// Single action should be an object, not an array
		$this->assertIsObject($obj->potentialAction);
		$this->assertEquals('SolveMathAction', $obj->potentialAction->{'@type'});
		$this->assertEquals('https://mathdomain.com/solve?q={math_expression_string}', $obj->potentialAction->target);
		$this->assertEquals('required name=math_expression_string', $obj->potentialAction->{'mathExpression-input'});
	}

	public function testArrayOfPotentialActions(): void {
		$action1 = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: 'Polynomial Equation',
		);

		$action2 = new SolveMathAction(
			target: 'https://mathdomain.com/quadratic?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: 'Quadratic Equation',
		);

		$schema = new MathSolver(
			url: 'https://www.mathdomain.com/',
			usageInfo: 'https://www.mathdomain.com/privacy',
			potentialAction: [$action1, $action2],
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		// Multiple actions should be an array
		$this->assertIsArray($obj->potentialAction);
		$this->assertCount(2, $obj->potentialAction);
		$this->assertEquals('SolveMathAction', $obj->potentialAction[0]->{'@type'});
		$this->assertEquals('SolveMathAction', $obj->potentialAction[1]->{'@type'});
		$this->assertEquals('Polynomial Equation', $obj->potentialAction[0]->eduQuestionType);
		$this->assertEquals('Quadratic Equation', $obj->potentialAction[1]->eduQuestionType);
	}

	public function testFullOutput(): void {
		$action = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: 'Polynomial Equation',
		);

		$schema = new MathSolver(
			url: 'https://www.mathdomain.com/',
			usageInfo: 'https://www.mathdomain.com/privacy',
			potentialAction: [$action],
			name: 'An awesome math solver',
			inLanguage: 'en',
			learningResourceType: 'Math solver',
			assesses: ['Polynomial equations', 'Linear equations'],
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertIsArray($obj->{'@type'});
		$this->assertEquals('MathSolver', $obj->{'@type'}[0]);
		$this->assertEquals('LearningResource', $obj->{'@type'}[1]);
		$this->assertEquals('An awesome math solver', $obj->name);
		$this->assertEquals('https://www.mathdomain.com/', $obj->url);
		$this->assertEquals('https://www.mathdomain.com/privacy', $obj->usageInfo);
		$this->assertEquals('en', $obj->inLanguage);
		$this->assertEquals('Math solver', $obj->learningResourceType);
		$this->assertIsArray($obj->assesses);
		$this->assertCount(2, $obj->assesses);
		$this->assertEquals('Polynomial equations', $obj->assesses[0]);
		$this->assertEquals('Linear equations', $obj->assesses[1]);
		$this->assertIsArray($obj->potentialAction);
		$this->assertCount(1, $obj->potentialAction);
	}

	public function testMatchesGoogleExample(): void {
		$action = new SolveMathAction(
			target: 'https://mathdomain.com/solve?q={math_expression_string}',
			mathExpressionInput: 'required name=math_expression_string',
			eduQuestionType: 'Polynomial Equation',
		);

		$schema = new MathSolver(
			url: 'https://www.mathdomain.com/',
			usageInfo: 'https://www.mathdomain.com/privacy',
			potentialAction: [$action],
			name: 'An awesome math solver',
			inLanguage: 'en',
			learningResourceType: 'Math solver',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		// Verify the structure matches the Google example
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertIsArray($obj->{'@type'});
		$this->assertEquals(['MathSolver', 'LearningResource'], $obj->{'@type'});
		$this->assertEquals('An awesome math solver', $obj->name);
		$this->assertEquals('https://www.mathdomain.com/', $obj->url);
		$this->assertEquals('https://www.mathdomain.com/privacy', $obj->usageInfo);
		$this->assertEquals('en', $obj->inLanguage);
		$this->assertIsArray($obj->potentialAction);
		$this->assertEquals('https://mathdomain.com/solve?q={math_expression_string}', $obj->potentialAction[0]->target);
		$this->assertEquals('required name=math_expression_string', $obj->potentialAction[0]->{'mathExpression-input'});
		$this->assertEquals('Polynomial Equation', $obj->potentialAction[0]->eduQuestionType);
		$this->assertEquals('Math solver', $obj->learningResourceType);
	}
}
