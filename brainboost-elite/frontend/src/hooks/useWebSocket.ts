import { useEffect, useRef, useState, useCallback } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { useMetricsStore } from '@/stores/metricsStore';
import { useProgressStore } from '@/stores/progressStore';

interface ServerMessage {
  type: string;
  payload: any;
}

export function useWebSocket() {
  const [isConnected, setIsConnected] = useState(false);
  const [lastMessage, setLastMessage] = useState<ServerMessage | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>();
  const reconnectAttempts = useRef(0);
  const pingIntervalRef = useRef<NodeJS.Timeout>();
  
  const { accessToken } = useAuthStore();
  const { setMetrics } = useMetricsStore();
  const { setProgress } = useProgressStore();

  const connect = useCallback(() => {
    if (!accessToken) return;

    const wsUrl = `${process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3001'}/ws?token=${accessToken}`;
    const ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      console.log('WebSocket connected');
      setIsConnected(true);
      reconnectAttempts.current = 0;

      // Start heartbeat
      pingIntervalRef.current = setInterval(() => {
        if (ws.readyState === WebSocket.OPEN) {
          ws.send(JSON.stringify({ type: 'Ping' }));
        }
      }, 25000);
    };

    ws.onmessage = (event) => {
      try {
        const message: ServerMessage = JSON.parse(event.data);
        setLastMessage(message);

        // Handle different message types
        switch (message.type) {
          case 'MetricsUpdate':
            setMetrics(message.payload);
            break;
          
          case 'StreakUpdate':
            setProgress((prev) => ({
              ...prev,
              streak: message.payload.current_streak,
            }));
            break;
          
          case 'SessionReminder':
            // Show toast notification
            if ('Notification' in window && Notification.permission === 'granted') {
              new Notification('BrainBoost Elite', {
                body: message.payload.message,
                icon: '/icons/brain-192.png',
                vibrate: [100, 50, 100],
              });
            }
            break;
          
          case 'AchievementUnlocked':
            // Trigger achievement animation
            console.log('Achievement unlocked:', message.payload);
            break;
          
          case 'StressShieldAlert':
            // Trigger NSDR modal if auto_nsdr is true
            console.log('Stress shield alert:', message.payload);
            break;
          
          case 'LevelAdvancement':
            // Trigger level unlock animation
            console.log('Level advancement:', message.payload);
            break;
          
          case 'DayMissedWarning':
            // Show warning modal
            console.log('Day missed:', message.payload);
            break;
          
          case 'Pong':
            // Heartbeat response
            break;
        }
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    ws.onclose = () => {
      console.log('WebSocket disconnected');
      setIsConnected(false);
      
      if (pingIntervalRef.current) {
        clearInterval(pingIntervalRef.current);
      }

      // Exponential backoff reconnection
      const delay = Math.min(30000, 1000 * Math.pow(2, reconnectAttempts.current));
      reconnectAttempts.current++;
      
      reconnectTimeoutRef.current = setTimeout(() => {
        console.log(`Reconnecting... (attempt ${reconnectAttempts.current})`);
        connect();
      }, delay);
    };

    wsRef.current = ws;
  }, [accessToken, setMetrics, setProgress]);

  useEffect(() => {
    connect();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
      if (pingIntervalRef.current) {
        clearInterval(pingIntervalRef.current);
      }
    };
  }, [connect]);

  const sendMessage = useCallback((message: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  }, []);

  return {
    isConnected,
    lastMessage,
    sendMessage,
  };
}
