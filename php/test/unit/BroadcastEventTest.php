<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BroadcastEvent;
use PHPUnit\Framework\TestCase;

final class BroadcastEventTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new BroadcastEvent(isLiveBroadcast: true);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('BroadcastEvent', $obj->{'@type'});
		$this->assertTrue($obj->isLiveBroadcast);
		$this->assertObjectNotHasProperty('startDate', $obj);
		$this->assertObjectNotHasProperty('endDate', $obj);
	}

	public function testFullOutput(): void {
		$schema = new BroadcastEvent(
			isLiveBroadcast: true,
			startDate: '2026-02-24T20:00:00+00:00',
			endDate: '2026-02-24T21:00:00+00:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertTrue($obj->isLiveBroadcast);
		$this->assertEquals('2026-02-24T20:00:00+00:00', $obj->startDate);
		$this->assertEquals('2026-02-24T21:00:00+00:00', $obj->endDate);
	}

	public function testFalseLiveBroadcastIsSerialized(): void {
		$schema = new BroadcastEvent(isLiveBroadcast: false);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse($obj->isLiveBroadcast);
		$this->assertObjectNotHasProperty('startDate', $obj);
		$this->assertObjectNotHasProperty('endDate', $obj);
	}

	public function testOnlyStartDateIsSerialized(): void {
		$schema = new BroadcastEvent(
			isLiveBroadcast: true,
			startDate: '2026-02-24T20:00:00+00:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('2026-02-24T20:00:00+00:00', $obj->startDate);
		$this->assertObjectNotHasProperty('endDate', $obj);
	}

	public function testOnlyEndDateIsSerialized(): void {
		$schema = new BroadcastEvent(
			isLiveBroadcast: true,
			endDate: '2026-02-24T21:00:00+00:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('2026-02-24T21:00:00+00:00', $obj->endDate);
		$this->assertObjectNotHasProperty('startDate', $obj);
	}
}
