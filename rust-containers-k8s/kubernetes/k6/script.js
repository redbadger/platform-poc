import http from "k6/http";
import { sleep, check } from "k6";

export const options = {
  vus: 100,
  thresholds: {
    // http errors should be less than 1%, otherwise abort the test
    http_req_failed: [{ threshold: "rate<0.01", abortOnFail: true }],
    // 99% of requests should be below 200ms
    http_req_duration: ["p(99)<200"],
  },
  scenarios: {
    average_load: {
      executor: "ramping-vus",
      stages: [
        { duration: "5s", target: 50 },
        { duration: "10s", target: 100 },
        { duration: "10s", target: 200 },
        { duration: "10s", target: 300 },
        { duration: "10s", target: 400 },
        { duration: "10s", target: 500 },
        { duration: "10s", target: 0 },
      ],
    },
  },
};

export default function () {
  const url = "http://localhost:8081/api/order";
  const payload = JSON.stringify({
    items: [{ sku: "iphone_13", price: 1, quantity: 1 }],
  });
  const params = {
    headers: {
      "Content-Type": "application/json",
    },
  };
  check(http.post(url, payload, params), {
    "status is 201": (res) => res.status === 201,
  });

  check(http.get(url), {
    "status is 200": (res) => res.status === 200,
  });

  sleep(1);
}
