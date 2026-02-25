<?php

namespace EvaLok\SchemaOrgJsonLd\v1;

use UnitEnum;

class JsonLdGenerator {
	public static function SchemaToJson(
		TypedSchema $schema,
	): string {
		$obj = self::SchemaToObject($schema);

		return json_encode($obj, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES);
	}

	public static function SchemaToObject(
		TypedSchema $schema,
		$initialContext = true,
	): array {
		$obj = [];

		if ($initialContext) {
			$obj['@context'] = "https://schema.org/";
		}

		$obj['@type'] = $schema::A_SCHEMA_TYPE;

		self::AddPropertiesToObject(get_object_vars($schema), $obj);

		return $obj;
	}

	private static function AddPropertiesToObject(
		array $properties,
		array &$obj,
	): void {

		foreach ($properties as $k => $v) {
			if (is_null($v)) {
				continue;
			} elseif (is_string($v) || is_numeric($v) || is_bool($v)) {
				$obj[$k] = $v;
			} elseif ($v instanceof UnitEnum) {
				$obj[$k] = $v->value;
			} elseif ($v instanceof TypedSchema) {
				$obj[$k] = self::SchemaToObject(
					schema: $v,
					initialContext: false,
				);
			} elseif (is_array($v)) {
				if (empty($v)) {
					continue;
				}

				if ($v[0] instanceof TypedSchema) {
					foreach ($v as $schema) {
						$obj[$k][] = self::SchemaToObject(
							schema: $schema,
							initialContext: false,
						);
					}
				} else {
					foreach ($v as $element) {
						if (is_string($element) || is_numeric($element) || is_bool($element)) {
							$obj[$k][] = $element;
						} elseif ($element instanceof UnitEnum) {
							$obj[$k][] = $element->value;
						}
					}
				}
			}
		}
	}
}
