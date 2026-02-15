// Service Worker for BrainBoost Elite PWA

const CACHE_NAME = 'brainboost-v1';
const OFFLINE_URL = '/offline';

// Assets to pre-cache for offline support
const PRECACHE_ASSETS = [
  '/',
  '/offline',
  '/manifest.json',
  '/icons/brain-192.png',
  '/icons/brain-512.png',
];

// Install: pre-cache core assets
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(PRECACHE_ASSETS))
  );
  self.skipWaiting();
});

// Activate: clean old caches
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k)))
    )
  );
  self.clients.claim();
});

// Fetch: network-first for API, cache-first for static assets
self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);

  // API calls: network only (no caching API responses)
  if (url.pathname.startsWith('/api/')) {
    event.respondWith(
      fetch(event.request).catch(() =>
        new Response(JSON.stringify({ error: 'offline' }), {
          status: 503,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );
    return;
  }

  // Static assets: cache-first
  event.respondWith(
    caches.match(event.request).then((cached) => {
      if (cached) return cached;
      return fetch(event.request).then((response) => {
        if (response.ok && response.type === 'basic') {
          const clone = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
        }
        return response;
      }).catch(() => {
        if (event.request.mode === 'navigate') {
          return caches.match(OFFLINE_URL);
        }
        return new Response('Offline', { status: 503 });
      });
    })
  );
});

// Push notifications
self.addEventListener('push', (event) => {
  let data = { title: 'BrainBoost Elite', body: 'Time for your session!' };
  try {
    data = event.data.json();
  } catch (e) {
    // Use defaults
  }

  const options = {
    body: data.body,
    icon: data.icon || '/icons/brain-192.png',
    badge: data.badge || '/icons/badge-72.png',
    vibrate: [100, 50, 100],
    data: data.data || {},
    actions: [
      { action: 'start', title: 'Start Session' },
      { action: 'snooze', title: 'Snooze 5 min' },
    ],
    tag: data.tag || 'session-reminder',
    renotify: true,
  };

  event.waitUntil(self.registration.showNotification(data.title, options));
});

// Notification click handling
self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  if (event.action === 'start') {
    event.waitUntil(clients.openWindow('/dashboard'));
  } else if (event.action === 'snooze') {
    // Re-notify in 5 minutes
    setTimeout(() => {
      self.registration.showNotification('BrainBoost Elite', {
        body: 'Your session is waiting!',
        icon: '/icons/brain-192.png',
        tag: 'session-reminder-snooze',
      });
    }, 5 * 60 * 1000);
  } else {
    event.waitUntil(clients.openWindow('/dashboard'));
  }
});
