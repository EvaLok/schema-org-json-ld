<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class FAQPage extends TypedSchema {

	const A_SCHEMA_TYPE = 'FAQPage';

	public function __construct(
		/** @var Question[] $mainEntity */
		public array $mainEntity,
	) {

	}
}
