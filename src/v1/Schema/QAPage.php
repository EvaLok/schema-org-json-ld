<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class QAPage extends TypedSchema {
	public const A_SCHEMA_TYPE = 'QAPage';

	public function __construct(
		public Question $mainEntity,
	) {}
}
