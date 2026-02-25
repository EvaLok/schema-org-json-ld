<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class SpeakableSpecification extends TypedSchema {
	public const A_SCHEMA_TYPE = 'SpeakableSpecification';

	public function __construct(
		/** @var null|string|string[] $cssSelector */
		public null|string|array $cssSelector = null,
		/** @var null|string|string[] $xpath */
		public null|string|array $xpath = null,
	) {}
}
