package main

import (
	"context"
	"fmt"
	"log"
	"math"

	"github.com/gofiber/fiber/v2"
	"google.golang.org/api/option"
	"google.golang.org/api/sheets/v4"
)

type Output struct {
	ValueToString string  `json:"value_to_string"`
	Integer       int     `json:"integer"`
	Float         float32 `json:"float"`
	Boolean       bool    `json:"boolean"`
}

func v(v int) Output {
	return Output{
		ValueToString: fmt.Sprintf("%d", v),
		Integer:       v,
		Float:         float32(v),
		Boolean:       v%2 == 0,
	}
}
func f(f float32) Output {
	return Output{
		ValueToString: fmt.Sprintf("%f", f),
		Integer:       int(f),
		Float:         f,
		Boolean:       false,
	}
}
func custom() Output {
	return Output{
		ValueToString: "0.3333333333333",
		Integer:       0,
		Float:         0.33333333333333333,
		Boolean:       true,
	}
}

func (o Output) out() []interface{} {
	return []interface {
	}{
		o.ValueToString,
		o.Integer,
		o.Float,
		o.Boolean,
	}
}

func main() {

	output := []Output{
		v(1),
		v(2),
		v(30),
		custom(),
		f(1.1),
		f(12.3),
		f(16.77),
		f(math.E),
		f(math.Pi),
	}

	ctx := context.Background()
	sheetsService, err := sheets.NewService(ctx, option.WithCredentialsFile("credentials.json"))
	if err != nil {
		log.Fatal(err)
	}

	spreadsheetId := "117NexZp-3BStfUbKPvqpVPJc1N9-YVAiwgLliIJ_-UE"

	spsOutput := []interface{}{
		"Value to string",
		"Integer",
		"Float",
		"Boolean",
	}

	_, err = sheetsService.Spreadsheets.Values.Append(spreadsheetId, "api!A1:A", &sheets.ValueRange{
		MajorDimension: "ROWS",
		Range:          "api!A1:A",
		Values: [][]interface{}{
			spsOutput,
			v(1).out(),
			v(2).out(),
			v(30).out(),
			v(100234).out(),
			v(125555).out(),
			v(1234567).out(),
			v(12345678).out(),
			Output{
				ValueToString: "13.44",
			}.out(),
			[]interface{}{"0.3333333333333", 0, 0.33333333333333333, true},
			f(1.1).out(),
			f(12.3).out(),
			f(16.77).out(),
			f(math.E).out(),
			f(math.Pi).out(),
		},
	}).ValueInputOption("USER_ENTERED").Do()

	fmt.Println(err)

	app := fiber.New()

	app.Get("/echo", func(c *fiber.Ctx) error {

		return c.JSON(output)
	})

	app.Listen(":8080")
}
