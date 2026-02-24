<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class HowToStep extends TypedSchema {
	public const A_SCHEMA_TYPE = 'HowToStep';

	public function __construct(
		public string $text,
		public null|string $name = null,
		public null|string $url = null,
		public null|string $image = null,
	) {}
}
