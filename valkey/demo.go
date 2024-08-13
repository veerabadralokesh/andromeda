package main

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/valkey-io/valkey-go"
)

func main() {
	// client, err := valkey.NewClient(valkey.ClientOption{InitAddress: []string{"127.0.0.1:6379"}, BlockingPoolSize: 100})
	client, err := valkey.NewClient(valkey.ClientOption{
		InitAddress:           []string{"127.0.0.1:6379"},
		ClientTrackingOptions: []string{"PREFIX", "prefix1:", "PREFIX", "prefix2:", "BCAST"},
	})
	processError(err)
	defer client.Close()

	ctx := context.Background()
	// SET key val NX
	err = client.Do(ctx, client.B().Set().Key("key").Value("val").Nx().Build()).Error()
	processError(err)

	// HGETALL hm
	hm, err := client.Do(ctx, client.B().Hgetall().Key("hm").Build()).AsStrMap()
	processError(err)

	fmt.Println(hm)

	cmd := client.B().Publish().Channel("channel1").Message("message").Build()
	err = client.Do(ctx, cmd).Error()
	processError(err)

	cmds := make(valkey.Commands, 0, 10)
	for i := 0; i < 10; i++ {
		// cmds = append(cmds, client.B().Set().Key("key").Value("value").Build())
		cmds = append(cmds, client.B().Publish().Channel("channel1").Message(fmt.Sprintf("%s-%d", "message", i)).Build())
	}
	for _, resp := range client.DoMulti(ctx, cmds...) {
		if err := resp.Error(); err != nil {
			panic(err)
		}
	}

	client.DoCache(ctx, client.B().Hmget().Key("mk").Field("1", "2").Cache(), time.Minute).ToArray()
	client.DoMultiCache(ctx,
		valkey.CT(client.B().Get().Key("k1").Cache(), 1*time.Minute),
		valkey.CT(client.B().Get().Key("k2").Cache(), 2*time.Minute))
	fmt.Println(client.DoCache(ctx, client.B().Get().Key("k1").Cache(), time.Minute).CacheTTL() == 60)
	fmt.Println(client.DoCache(ctx, client.B().Get().Key("k1").Cache(), time.Minute).IsCacheHit())

	fmt.Println(client.DoCache(ctx, client.B().Get().Key("prefix1:1").Cache(), time.Minute).IsCacheHit())
	fmt.Println(client.DoCache(ctx, client.B().Get().Key("prefix1:1").Cache(), time.Minute).IsCacheHit())

}

func BenchmarkPipelining(b *testing.B, client valkey.Client) {
	// the below client.Do() operations will be issued from
	// multiple goroutines and thus will be pipelined automatically.
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			client.Do(context.Background(), client.B().Get().Key("k").Build()).ToString()
		}
	})
}

func processError(err error) {
	if err == valkey.Nil {
		return
	}
	if err != nil {
		panic(err)
	}
}
