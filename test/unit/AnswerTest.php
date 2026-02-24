<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Answer;
use PHPUnit\Framework\TestCase;

final class AnswerTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Answer(text: 'You can return items within 30 days.');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Answer', $obj->{'@type'});
		$this->assertEquals('You can return items within 30 days.', $obj->text);
	}
}
