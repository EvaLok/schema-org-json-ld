<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum EventStatusType: string {
	case EventScheduled = 'https://schema.org/EventScheduled';
	case EventCancelled = 'https://schema.org/EventCancelled';
	case EventPostponed = 'https://schema.org/EventPostponed';
	case EventRescheduled = 'https://schema.org/EventRescheduled';
}
