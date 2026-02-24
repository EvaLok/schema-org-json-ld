<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class DataDownload extends TypedSchema {
	public const A_SCHEMA_TYPE = 'DataDownload';

	public function __construct(
		public string $contentUrl,
		public null|string $encodingFormat = null,
	) {}
}
