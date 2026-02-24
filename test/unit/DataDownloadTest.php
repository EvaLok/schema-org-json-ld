<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DataDownload;
use PHPUnit\Framework\TestCase;

final class DataDownloadTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new DataDownload(contentUrl: 'https://example.com/data.csv');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('DataDownload', $obj->{'@type'});
		$this->assertEquals('https://example.com/data.csv', $obj->contentUrl);
		$this->assertObjectNotHasProperty('encodingFormat', $obj);
	}

	public function testFullOutput(): void {
		$schema = new DataDownload(
			contentUrl: 'https://example.com/data.csv',
			encodingFormat: 'text/csv',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);
		$this->assertEquals('https://example.com/data.csv', $obj->contentUrl);
		$this->assertEquals('text/csv', $obj->encodingFormat);
	}
}
