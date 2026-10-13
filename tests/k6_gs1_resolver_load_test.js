// Grafana k6 Official Concurrent Load Test Script for Viettel GS1 Digital Link Resolver (RFC 9264)
// Run with: k6 run tests/k6_gs1_resolver_load_test.js

import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '5s', target: 20 },  // Ramp-up to 20 concurrent VUs
    { duration: '15s', target: 50 }, // Sustained load at 50 concurrent VUs
    { duration: '5s', target: 0 },   // Ramp-down
  ],
  thresholds: {
    http_req_duration: ['p(95)<20', 'p(99)<50'], // 95% of requests must complete below 20ms
    http_req_failed: ['rate<0.01'],              // Error rate below 1%
  },
};

export default function () {
  const gtin = '09501101420014';
  const batch = 'LOT-VN-2026-09';
  const serial = `BOX-${Math.floor(Math.random() * 1000) + 1}`;
  const url = `http://127.0.0.1:8090/01/${gtin}/10/${batch}/21/${serial}`;

  const params = {
    headers: {
      'Accept': 'application/linkset+json',
      'User-Agent': 'k6-viettel-gs1-load-test/1.0',
    },
  };

  const res = http.get(url, params);

  check(res, {
    'status is 200': (r) => r.status === 200,
    'content-type is RFC 9264 linkset': (r) => r.headers['Content-Type'] && r.headers['Content-Type'].includes('application/linkset+json'),
    'has valid linkset anchor': (r) => {
      try {
        const json = JSON.parse(r.body);
        return json.linkset && json.linkset.length > 0 && json.linkset[0].anchor.includes(gtin);
      } catch (e) {
        return false;
      }
    },
  });

  sleep(0.01);
}
