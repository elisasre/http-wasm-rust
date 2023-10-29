package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/http-wasm/http-wasm-host-go/handler"
	wasm "github.com/http-wasm/http-wasm-host-go/handler/nethttp"
	"github.com/rs/zerolog"
	log "github.com/rs/zerolog/log"
	"github.com/tetratelabs/wazero"
)

func helloWorld(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte(fmt.Sprintf("Method: %s\n", r.Method)))
	w.Write([]byte(fmt.Sprintf("URI: %s\n", r.URL.String())))

	b, err := io.ReadAll(r.Body)
	if err == nil {
		w.Write([]byte(fmt.Sprintf("Body: %s\n", b)))
	}

	for name, values := range r.Header {
		// Loop over all values for the name.
		for _, value := range values {
			w.Write([]byte(fmt.Sprintf("%s: %s\n", name, value)))
		}
	}
}

func main() {
	handler, err := makeWasmHandler(http.HandlerFunc(helloWorld))
	if err != nil {
		panic(err)
	}
	http.Handle("/hello", handler)
	log.Info().Msg("Listening on :8090")
	http.ListenAndServe(":8090", nil)
}

func makeWasmHandler(next http.Handler) (http.Handler, error) {
	code, err := os.ReadFile("../header.wasm")
	if err != nil {
		return nil, err
	}

	config := map[string]interface{}{
		"headers": map[string]string{
			"X-foo": "Hello, World!",
		},
	}

	b, err := json.Marshal(config)
	if err != nil {
		return nil, err
	}

	logger := zerolog.New(os.Stdout).With().Timestamp().Logger()
	opts := []handler.Option{
		handler.ModuleConfig(wazero.NewModuleConfig().WithSysWalltime()),
		handler.Logger(initWasmLogger(&logger)),
		handler.GuestConfig(b),
	}

	mw, err := wasm.NewMiddleware(context.Background(), code, opts...)
	if err != nil {
		return nil, err
	}
	return mw.NewHandler(context.Background(), next), nil
}
