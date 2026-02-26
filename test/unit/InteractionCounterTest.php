<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\InteractionCounter;
use PHPUnit\Framework\TestCase;

final class InteractionCounterTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new InteractionCounter(
			interactionType: 'https://schema.org/FollowAction',
			userInteractionCount: 1234,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('InteractionCounter', $obj->{'@type'});
		$this->assertEquals('https://schema.org/FollowAction', $obj->interactionType);
		$this->assertEquals(1234, $obj->userInteractionCount);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new InteractionCounter(
			interactionType: 'https://schema.org/LikeAction',
			userInteractionCount: 42,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('interactionService', $obj);
	}

	public function testFullOutput(): void {
		$schema = new InteractionCounter(
			interactionType: 'https://schema.org/WriteAction',
			userInteractionCount: 7,
			interactionService: 'https://example.com',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/WriteAction', $obj->interactionType);
		$this->assertEquals(7, $obj->userInteractionCount);
		$this->assertEquals('https://example.com', $obj->interactionService);
	}
}
