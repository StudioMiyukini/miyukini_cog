import { NextRequest, NextResponse } from 'next/server'

const SUPABASE_URL = process.env.NEXT_PUBLIC_SUPABASE_URL!

export async function POST(req: NextRequest) {
  try {
    const authHeader = req.headers.get('authorization')
    if (!authHeader) {
      return NextResponse.json({ error: 'unauthorized' }, { status: 401 })
    }

    const body = await req.json()
    const { plan_id } = body

    if (!plan_id) {
      return NextResponse.json({ error: 'plan_id_required' }, { status: 400 })
    }

    // Appeler l'Edge Function Supabase
    const response = await fetch(`${SUPABASE_URL}/functions/v1/paywall-create-plan`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: authHeader,
      },
      body: JSON.stringify({ plan_id }),
    })

    const data = await response.json()

    if (!response.ok) {
      return NextResponse.json(data, { status: response.status })
    }

    return NextResponse.json(data)
  } catch (error) {
    return NextResponse.json(
      { error: 'unexpected', details: error instanceof Error ? error.message : String(error) },
      { status: 500 }
    )
  }
}
