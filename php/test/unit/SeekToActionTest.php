<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SeekToAction;
use PHPUnit\Framework\TestCase;

final class SeekToActionTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new SeekToAction(
			target: 'https://example.com/watch?v=abc&t={seek_to_second_number}',
			startOffsetInput: 'required name=seek_to_second_number',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('SeekToAction', $obj->{'@type'});
		$this->assertEquals('https://example.com/watch?v=abc&t={seek_to_second_number}', $obj->target);
		$this->assertEquals('required name=seek_to_second_number', $obj->{'startOffset-input'});
		$this->assertObjectNotHasProperty('startOffsetInput', $obj);
	}
}
