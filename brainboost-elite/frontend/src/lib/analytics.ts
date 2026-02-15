type EventName =
  | 'page_view'
  | 'session_started'
  | 'session_completed'
  | 'exercise_completed'
  | 'exercise_skipped'
  | 'level_advanced'
  | 'level_restarted'
  | 'streak_milestone'
  | 'achievement_unlocked'
  | 'trading_session_started'
  | 'trading_decision_made'
  | 'topic_created'
  | 'lesson_completed'
  | 'flashcard_reviewed'
  | 'coach_message_sent'
  | 'nsdr_triggered'
  | 'onboarding_completed'
  | 'onboarding_step'
  | 'subscription_upgrade'
  | 'notification_permission'
  | 'pwa_installed';

interface AnalyticsEvent {
  name: EventName;
  properties?: Record<string, string | number | boolean>;
}

class Analytics {
  private queue: AnalyticsEvent[] = [];
  private initialized = false;

  init() {
    if (typeof window === 'undefined') return;
    this.initialized = true;
    this.flush();
  }

  track(name: EventName, properties?: Record<string, string | number | boolean>) {
    const event: AnalyticsEvent = { name, properties };

    if (!this.initialized) {
      this.queue.push(event);
      return;
    }

    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', name, properties);
    }

    if (process.env.NODE_ENV === 'development') {
      console.log('[Analytics]', name, properties);
    }
  }

  private flush() {
    this.queue.forEach((event) => this.track(event.name, event.properties));
    this.queue = [];
  }
}

export const analytics = new Analytics();
