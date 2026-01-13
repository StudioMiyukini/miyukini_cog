export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

export type Database = {
  // Allows to automatically instantiate createClient with right options
  // instead of createClient<Database, { PostgrestVersion: 'XX' }>(URL, KEY)
  __InternalSupabase: {
    PostgrestVersion: "14.1"
  }
  public: {
    Tables: {
      agendas: {
        Row: {
          allow_overbooking: boolean | null
          created_at: string
          created_by: string
          default_slot_duration: number | null
          description: string | null
          id: string
          is_public: boolean
          max_participants_per_slot: number | null
          module_id: string
          name: string
          slot_buffer_after: number | null
          slot_buffer_before: number | null
          timezone: string | null
          updated_at: string
          workspace_id: string | null
        }
        Insert: {
          allow_overbooking?: boolean | null
          created_at?: string
          created_by: string
          default_slot_duration?: number | null
          description?: string | null
          id?: string
          is_public?: boolean
          max_participants_per_slot?: number | null
          module_id: string
          name: string
          slot_buffer_after?: number | null
          slot_buffer_before?: number | null
          timezone?: string | null
          updated_at?: string
          workspace_id?: string | null
        }
        Update: {
          allow_overbooking?: boolean | null
          created_at?: string
          created_by?: string
          default_slot_duration?: number | null
          description?: string | null
          id?: string
          is_public?: boolean
          max_participants_per_slot?: number | null
          module_id?: string
          name?: string
          slot_buffer_after?: number | null
          slot_buffer_before?: number | null
          timezone?: string | null
          updated_at?: string
          workspace_id?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "agendas_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_bookings: {
        Row: {
          agenda_id: string | null
          cancel_reason: string | null
          created_at: string
          customer_email: string | null
          customer_id: string | null
          customer_phone: string | null
          id: string
          notes_customer: string | null
          notes_internal: string | null
          provider_id: string
          quantity: number
          service_id: string
          slot_id: string | null
          status: Database["public"]["Enums"]["booking_booking_status"]
          updated_at: string
        }
        Insert: {
          agenda_id?: string | null
          cancel_reason?: string | null
          created_at?: string
          customer_email?: string | null
          customer_id?: string | null
          customer_phone?: string | null
          id?: string
          notes_customer?: string | null
          notes_internal?: string | null
          provider_id: string
          quantity?: number
          service_id: string
          slot_id?: string | null
          status?: Database["public"]["Enums"]["booking_booking_status"]
          updated_at?: string
        }
        Update: {
          agenda_id?: string | null
          cancel_reason?: string | null
          created_at?: string
          customer_email?: string | null
          customer_id?: string | null
          customer_phone?: string | null
          id?: string
          notes_customer?: string | null
          notes_internal?: string | null
          provider_id?: string
          quantity?: number
          service_id?: string
          slot_id?: string | null
          status?: Database["public"]["Enums"]["booking_booking_status"]
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "booking_bookings_customer_id_fkey"
            columns: ["customer_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "booking_bookings_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "booking_bookings_service_id_fkey"
            columns: ["service_id"]
            isOneToOne: false
            referencedRelation: "booking_services"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_provider_photos: {
        Row: {
          alt: string | null
          created_at: string
          id: string
          is_public: boolean
          path: string | null
          provider_id: string
          sort_order: number
          updated_at: string
          url: string | null
        }
        Insert: {
          alt?: string | null
          created_at?: string
          id?: string
          is_public?: boolean
          path?: string | null
          provider_id: string
          sort_order?: number
          updated_at?: string
          url?: string | null
        }
        Update: {
          alt?: string | null
          created_at?: string
          id?: string
          is_public?: boolean
          path?: string | null
          provider_id?: string
          sort_order?: number
          updated_at?: string
          url?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "booking_provider_photos_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_provider_reviews: {
        Row: {
          author_name: string | null
          booking_id: string | null
          comment: string | null
          created_at: string
          id: string
          is_published: boolean
          provider_id: string
          rating: number
        }
        Insert: {
          author_name?: string | null
          booking_id?: string | null
          comment?: string | null
          created_at?: string
          id?: string
          is_published?: boolean
          provider_id: string
          rating: number
        }
        Update: {
          author_name?: string | null
          booking_id?: string | null
          comment?: string | null
          created_at?: string
          id?: string
          is_published?: boolean
          provider_id?: string
          rating?: number
        }
        Relationships: [
          {
            foreignKeyName: "booking_provider_reviews_booking_id_fkey"
            columns: ["booking_id"]
            isOneToOne: false
            referencedRelation: "booking_bookings"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "booking_provider_reviews_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_providers: {
        Row: {
          address_line1: string | null
          address_line2: string | null
          city: string | null
          contact_email: string | null
          country: string | null
          created_at: string
          description: string | null
          display_name: string | null
          id: string
          is_active: boolean
          lat: number | null
          lng: number | null
          opening_hours: Json
          phone: string | null
          postal_code: string | null
          social_links: Json
          tags: string[] | null
          timezone: string | null
          updated_at: string
          website: string | null
        }
        Insert: {
          address_line1?: string | null
          address_line2?: string | null
          city?: string | null
          contact_email?: string | null
          country?: string | null
          created_at?: string
          description?: string | null
          display_name?: string | null
          id: string
          is_active?: boolean
          lat?: number | null
          lng?: number | null
          opening_hours?: Json
          phone?: string | null
          postal_code?: string | null
          social_links?: Json
          tags?: string[] | null
          timezone?: string | null
          updated_at?: string
          website?: string | null
        }
        Update: {
          address_line1?: string | null
          address_line2?: string | null
          city?: string | null
          contact_email?: string | null
          country?: string | null
          created_at?: string
          description?: string | null
          display_name?: string | null
          id?: string
          is_active?: boolean
          lat?: number | null
          lng?: number | null
          opening_hours?: Json
          phone?: string | null
          postal_code?: string | null
          social_links?: Json
          tags?: string[] | null
          timezone?: string | null
          updated_at?: string
          website?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "booking_providers_id_fkey"
            columns: ["id"]
            isOneToOne: true
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_services: {
        Row: {
          cancellation_policy: Json | null
          category: string | null
          category_sort: number | null
          created_at: string
          currency: string | null
          default_capacity: number
          description: string | null
          duration_minutes: number
          id: string
          is_active: boolean
          name: string
          price_hint: number | null
          provider_id: string
          requires_approval: boolean
          tags: string[] | null
          updated_at: string
        }
        Insert: {
          cancellation_policy?: Json | null
          category?: string | null
          category_sort?: number | null
          created_at?: string
          currency?: string | null
          default_capacity?: number
          description?: string | null
          duration_minutes: number
          id?: string
          is_active?: boolean
          name: string
          price_hint?: number | null
          provider_id: string
          requires_approval?: boolean
          tags?: string[] | null
          updated_at?: string
        }
        Update: {
          cancellation_policy?: Json | null
          category?: string | null
          category_sort?: number | null
          created_at?: string
          currency?: string | null
          default_capacity?: number
          description?: string | null
          duration_minutes?: number
          id?: string
          is_active?: boolean
          name?: string
          price_hint?: number | null
          provider_id?: string
          requires_approval?: boolean
          tags?: string[] | null
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "booking_services_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_slot_services: {
        Row: {
          created_at: string
          id: string
          provider_id: string
          service_id: string
          slot_id: string
        }
        Insert: {
          created_at?: string
          id?: string
          provider_id: string
          service_id: string
          slot_id: string
        }
        Update: {
          created_at?: string
          id?: string
          provider_id?: string
          service_id?: string
          slot_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "booking_slot_services_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "booking_slot_services_service_id_fkey"
            columns: ["service_id"]
            isOneToOne: false
            referencedRelation: "booking_services"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_time_off: {
        Row: {
          created_at: string
          end_at: string
          id: string
          mode: Database["public"]["Enums"]["booking_time_off_mode"]
          provider_id: string
          reason: string | null
          start_at: string
          updated_at: string
        }
        Insert: {
          created_at?: string
          end_at: string
          id?: string
          mode?: Database["public"]["Enums"]["booking_time_off_mode"]
          provider_id: string
          reason?: string | null
          start_at: string
          updated_at?: string
        }
        Update: {
          created_at?: string
          end_at?: string
          id?: string
          mode?: Database["public"]["Enums"]["booking_time_off_mode"]
          provider_id?: string
          reason?: string | null
          start_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "booking_time_off_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
        ]
      }
      booking_week_templates: {
        Row: {
          created_at: string
          id: string
          is_active: boolean
          name: string
          provider_id: string
          rules: Json
          timezone: string | null
          updated_at: string
        }
        Insert: {
          created_at?: string
          id?: string
          is_active?: boolean
          name: string
          provider_id: string
          rules?: Json
          timezone?: string | null
          updated_at?: string
        }
        Update: {
          created_at?: string
          id?: string
          is_active?: boolean
          name?: string
          provider_id?: string
          rules?: Json
          timezone?: string | null
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "booking_week_templates_provider_id_fkey"
            columns: ["provider_id"]
            isOneToOne: false
            referencedRelation: "booking_providers"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_invoice_payments: {
        Row: {
          amount: number
          confirmed_by: string | null
          created_at: string
          id: string
          invoice_id: string
          method: Database["public"]["Enums"]["commerce_payment_method"]
          notes: string | null
          paid_at: string
          proof_document_ref: Json | null
          reference: string | null
        }
        Insert: {
          amount: number
          confirmed_by?: string | null
          created_at?: string
          id?: string
          invoice_id: string
          method: Database["public"]["Enums"]["commerce_payment_method"]
          notes?: string | null
          paid_at?: string
          proof_document_ref?: Json | null
          reference?: string | null
        }
        Update: {
          amount?: number
          confirmed_by?: string | null
          created_at?: string
          id?: string
          invoice_id?: string
          method?: Database["public"]["Enums"]["commerce_payment_method"]
          notes?: string | null
          paid_at?: string
          proof_document_ref?: Json | null
          reference?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "commerce_invoice_payments_confirmed_by_fkey"
            columns: ["confirmed_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_invoice_payments_invoice_id_fkey"
            columns: ["invoice_id"]
            isOneToOne: false
            referencedRelation: "commerce_quote_invoices"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_outbox: {
        Row: {
          attempt_count: number
          created_at: string
          dedupe_key: string
          event_type: string
          id: string
          last_error: string | null
          next_retry_at: string | null
          payload: Json
          status: string
          updated_at: string
        }
        Insert: {
          attempt_count?: number
          created_at?: string
          dedupe_key: string
          event_type: string
          id?: string
          last_error?: string | null
          next_retry_at?: string | null
          payload?: Json
          status?: string
          updated_at?: string
        }
        Update: {
          attempt_count?: number
          created_at?: string
          dedupe_key?: string
          event_type?: string
          id?: string
          last_error?: string | null
          next_retry_at?: string | null
          payload?: Json
          status?: string
          updated_at?: string
        }
        Relationships: []
      }
      commerce_quote_audit: {
        Row: {
          actor_id: string | null
          created_at: string
          event_type: string
          id: string
          payload: Json
          quote_id: string
        }
        Insert: {
          actor_id?: string | null
          created_at?: string
          event_type: string
          id?: string
          payload?: Json
          quote_id: string
        }
        Update: {
          actor_id?: string | null
          created_at?: string
          event_type?: string
          id?: string
          payload?: Json
          quote_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "commerce_quote_audit_actor_id_fkey"
            columns: ["actor_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quote_audit_quote_id_fkey"
            columns: ["quote_id"]
            isOneToOne: false
            referencedRelation: "commerce_quotes"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_quote_invoices: {
        Row: {
          created_at: string
          created_by: string | null
          currency: string
          due_date: string | null
          id: string
          invoice_number: string
          issued_at: string | null
          quote_id: string
          requester_id: string | null
          status: Database["public"]["Enums"]["commerce_invoice_status"]
          subtotal_net: number | null
          tax_amount: number | null
          total_gross: number | null
          updated_at: string
        }
        Insert: {
          created_at?: string
          created_by?: string | null
          currency?: string
          due_date?: string | null
          id?: string
          invoice_number: string
          issued_at?: string | null
          quote_id: string
          requester_id?: string | null
          status?: Database["public"]["Enums"]["commerce_invoice_status"]
          subtotal_net?: number | null
          tax_amount?: number | null
          total_gross?: number | null
          updated_at?: string
        }
        Update: {
          created_at?: string
          created_by?: string | null
          currency?: string
          due_date?: string | null
          id?: string
          invoice_number?: string
          issued_at?: string | null
          quote_id?: string
          requester_id?: string | null
          status?: Database["public"]["Enums"]["commerce_invoice_status"]
          subtotal_net?: number | null
          tax_amount?: number | null
          total_gross?: number | null
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "commerce_quote_invoices_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quote_invoices_quote_id_fkey"
            columns: ["quote_id"]
            isOneToOne: false
            referencedRelation: "commerce_quotes"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quote_invoices_requester_id_fkey"
            columns: ["requester_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_quote_items: {
        Row: {
          created_at: string
          description: string | null
          id: string
          label: string
          line_total_net: number
          position: number
          quantity: number
          quote_id: string
          tax_rate: number
          unit_price_net: number
          updated_at: string
        }
        Insert: {
          created_at?: string
          description?: string | null
          id?: string
          label: string
          line_total_net?: number
          position?: number
          quantity?: number
          quote_id: string
          tax_rate?: number
          unit_price_net?: number
          updated_at?: string
        }
        Update: {
          created_at?: string
          description?: string | null
          id?: string
          label?: string
          line_total_net?: number
          position?: number
          quantity?: number
          quote_id?: string
          tax_rate?: number
          unit_price_net?: number
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "commerce_quote_items_quote_id_fkey"
            columns: ["quote_id"]
            isOneToOne: false
            referencedRelation: "commerce_quotes"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_quote_messages: {
        Row: {
          author_id: string | null
          author_role: Database["public"]["Enums"]["commerce_quote_message_author_role"]
          created_at: string
          id: string
          is_internal: boolean
          message: string
          quote_id: string
        }
        Insert: {
          author_id?: string | null
          author_role: Database["public"]["Enums"]["commerce_quote_message_author_role"]
          created_at?: string
          id?: string
          is_internal?: boolean
          message: string
          quote_id: string
        }
        Update: {
          author_id?: string | null
          author_role?: Database["public"]["Enums"]["commerce_quote_message_author_role"]
          created_at?: string
          id?: string
          is_internal?: boolean
          message?: string
          quote_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "commerce_quote_messages_author_id_fkey"
            columns: ["author_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quote_messages_quote_id_fkey"
            columns: ["quote_id"]
            isOneToOne: false
            referencedRelation: "commerce_quotes"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_quote_requests: {
        Row: {
          assigned_to: string | null
          category: string | null
          contact_email: string | null
          contact_phone: string | null
          created_at: string
          description: string | null
          id: string
          meta: Json
          preferred_contact_channel: string | null
          requested_by: string | null
          status: Database["public"]["Enums"]["commerce_quote_request_status"]
          tags: string[] | null
          title: string
          updated_at: string
          workspace_id: string | null
        }
        Insert: {
          assigned_to?: string | null
          category?: string | null
          contact_email?: string | null
          contact_phone?: string | null
          created_at?: string
          description?: string | null
          id?: string
          meta?: Json
          preferred_contact_channel?: string | null
          requested_by?: string | null
          status?: Database["public"]["Enums"]["commerce_quote_request_status"]
          tags?: string[] | null
          title: string
          updated_at?: string
          workspace_id?: string | null
        }
        Update: {
          assigned_to?: string | null
          category?: string | null
          contact_email?: string | null
          contact_phone?: string | null
          created_at?: string
          description?: string | null
          id?: string
          meta?: Json
          preferred_contact_channel?: string | null
          requested_by?: string | null
          status?: Database["public"]["Enums"]["commerce_quote_request_status"]
          tags?: string[] | null
          title?: string
          updated_at?: string
          workspace_id?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "commerce_quote_requests_assigned_to_fkey"
            columns: ["assigned_to"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quote_requests_requested_by_fkey"
            columns: ["requested_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      commerce_quotes: {
        Row: {
          accepted_at: string | null
          assigned_to: string | null
          created_at: string
          created_by: string | null
          currency: string
          id: string
          internal_notes: string | null
          quote_number: string
          quote_request_id: string
          rejected_at: string | null
          requester_id: string | null
          revision: number
          sent_at: string | null
          status: Database["public"]["Enums"]["commerce_quote_status"]
          subtotal_net: number | null
          tax_amount: number | null
          terms: string | null
          total_gross: number | null
          updated_at: string
          updated_by: string | null
          valid_until: string | null
          workspace_id: string | null
        }
        Insert: {
          accepted_at?: string | null
          assigned_to?: string | null
          created_at?: string
          created_by?: string | null
          currency?: string
          id?: string
          internal_notes?: string | null
          quote_number: string
          quote_request_id: string
          rejected_at?: string | null
          requester_id?: string | null
          revision?: number
          sent_at?: string | null
          status?: Database["public"]["Enums"]["commerce_quote_status"]
          subtotal_net?: number | null
          tax_amount?: number | null
          terms?: string | null
          total_gross?: number | null
          updated_at?: string
          updated_by?: string | null
          valid_until?: string | null
          workspace_id?: string | null
        }
        Update: {
          accepted_at?: string | null
          assigned_to?: string | null
          created_at?: string
          created_by?: string | null
          currency?: string
          id?: string
          internal_notes?: string | null
          quote_number?: string
          quote_request_id?: string
          rejected_at?: string | null
          requester_id?: string | null
          revision?: number
          sent_at?: string | null
          status?: Database["public"]["Enums"]["commerce_quote_status"]
          subtotal_net?: number | null
          tax_amount?: number | null
          terms?: string | null
          total_gross?: number | null
          updated_at?: string
          updated_by?: string | null
          valid_until?: string | null
          workspace_id?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "commerce_quotes_assigned_to_fkey"
            columns: ["assigned_to"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quotes_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quotes_quote_request_id_fkey"
            columns: ["quote_request_id"]
            isOneToOne: false
            referencedRelation: "commerce_quote_requests"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quotes_requester_id_fkey"
            columns: ["requester_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "commerce_quotes_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      categories: {
        Row: {
          created_at: string
          icon_class: string
          id: string
          is_default: boolean | null
          is_visible: boolean
          name: string
          path: string
          sort_order: number
          updated_at: string
        }
        Insert: {
          created_at?: string
          icon_class: string
          id: string
          is_default?: boolean | null
          is_visible?: boolean
          name: string
          path: string
          sort_order?: number
          updated_at?: string
        }
        Update: {
          created_at?: string
          icon_class?: string
          id?: string
          is_default?: boolean | null
          is_visible?: boolean
          name?: string
          path?: string
          sort_order?: number
          updated_at?: string
        }
        Relationships: []
      }
      framework_modules: {
        Row: {
          created_at: string
          enabled: boolean
          module_id: string
          updated_at: string
        }
        Insert: {
          created_at?: string
          enabled?: boolean
          module_id: string
          updated_at?: string
        }
        Update: {
          created_at?: string
          enabled?: boolean
          module_id?: string
          updated_at?: string
        }
        Relationships: []
      }
      app_branding: {
        Row: {
          id: string
          app_title: string
          logo_path: string | null
          logo_url: string | null
          updated_at: string
          updated_by: string | null
        }
        Insert: {
          id?: string
          app_title?: string
          logo_path?: string | null
          logo_url?: string | null
          updated_at?: string
          updated_by?: string | null
        }
        Update: {
          id?: string
          app_title?: string
          logo_path?: string | null
          logo_url?: string | null
          updated_at?: string
          updated_by?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "app_branding_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      homepage_content: {
        Row: {
          config: Json
          created_at: string
          id: string
          is_published: boolean
          updated_at: string
          updated_by: string | null
        }
        Insert: {
          config?: Json
          created_at?: string
          id: string
          is_published?: boolean
          updated_at?: string
          updated_by?: string | null
        }
        Update: {
          config?: Json
          created_at?: string
          id?: string
          is_published?: boolean
          updated_at?: string
          updated_by?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "homepage_content_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
        ]
      }
      profiles: {
        Row: {
          avatar_url: string | null
          created_at: string
          deleted_at: string | null
          display_name: string | null
          email: string | null
          email_verified: boolean | null
          first_name: string | null
          id: string
          last_name: string | null
          metadata: Json | null
          onboarding_completed: boolean | null
          phone: string | null
          phone_verified: boolean | null
          role: Database["public"]["Enums"]["user_role"]
          tier: Database["public"]["Enums"]["user_tier"]
          updated_at: string
        }
        Insert: {
          avatar_url?: string | null
          created_at?: string
          deleted_at?: string | null
          display_name?: string | null
          email?: string | null
          email_verified?: boolean | null
          first_name?: string | null
          id: string
          last_name?: string | null
          metadata?: Json | null
          onboarding_completed?: boolean | null
          phone?: string | null
          phone_verified?: boolean | null
          role?: Database["public"]["Enums"]["user_role"]
          tier?: Database["public"]["Enums"]["user_tier"]
          updated_at?: string
        }
        Update: {
          avatar_url?: string | null
          created_at?: string
          deleted_at?: string | null
          display_name?: string | null
          email?: string | null
          email_verified?: boolean | null
          first_name?: string | null
          id?: string
          last_name?: string | null
          metadata?: Json | null
          onboarding_completed?: boolean | null
          phone?: string | null
          phone_verified?: boolean | null
          role?: Database["public"]["Enums"]["user_role"]
          tier?: Database["public"]["Enums"]["user_tier"]
          updated_at?: string
        }
        Relationships: []
      }
      user_category_preferences: {
        Row: {
          category_id: string
          created_at: string
          custom_order: number | null
          enabled: boolean
          id: string
          updated_at: string
          user_id: string
        }
        Insert: {
          category_id: string
          created_at?: string
          custom_order?: number | null
          enabled?: boolean
          id?: string
          updated_at?: string
          user_id: string
        }
        Update: {
          category_id?: string
          created_at?: string
          custom_order?: number | null
          enabled?: boolean
          id?: string
          updated_at?: string
          user_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "user_category_preferences_category_id_fkey"
            columns: ["category_id"]
            isOneToOne: false
            referencedRelation: "categories"
            referencedColumns: ["id"]
          },
        ]
      }
      user_consents: {
        Row: {
          consent_type: Database["public"]["Enums"]["consent_type"]
          created_at: string
          granted: boolean
          granted_at: string | null
          id: string
          ip_address: unknown
          revoked_at: string | null
          updated_at: string
          user_agent: string | null
          user_id: string
        }
        Insert: {
          consent_type: Database["public"]["Enums"]["consent_type"]
          created_at?: string
          granted?: boolean
          granted_at?: string | null
          id?: string
          ip_address?: unknown
          revoked_at?: string | null
          updated_at?: string
          user_agent?: string | null
          user_id: string
        }
        Update: {
          consent_type?: Database["public"]["Enums"]["consent_type"]
          created_at?: string
          granted?: boolean
          granted_at?: string | null
          id?: string
          ip_address?: unknown
          revoked_at?: string | null
          updated_at?: string
          user_agent?: string | null
          user_id?: string
        }
        Relationships: []
      }
      slot_events: {
        Row: {
          actor_id: string | null
          created_at: string
          event_type: string
          id: string
          payload: Json | null
          slot_id: string
        }
        Insert: {
          actor_id?: string | null
          created_at?: string
          event_type: string
          id?: string
          payload?: Json | null
          slot_id: string
        }
        Update: {
          actor_id?: string | null
          created_at?: string
          event_type?: string
          id?: string
          payload?: Json | null
          slot_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "slot_events_actor_id_fkey"
            columns: ["actor_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "slot_events_slot_id_fkey"
            columns: ["slot_id"]
            isOneToOne: false
            referencedRelation: "slots"
            referencedColumns: ["id"]
          },
        ]
      }
      slot_participants: {
        Row: {
          confirmed_at: string | null
          created_at: string
          id: string
          role: string | null
          slot_id: string
          status: string | null
          user_id: string
        }
        Insert: {
          confirmed_at?: string | null
          created_at?: string
          id?: string
          role?: string | null
          slot_id: string
          status?: string | null
          user_id: string
        }
        Update: {
          confirmed_at?: string | null
          created_at?: string
          id?: string
          role?: string | null
          slot_id?: string
          status?: string | null
          user_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "slot_participants_slot_id_fkey"
            columns: ["slot_id"]
            isOneToOne: false
            referencedRelation: "slots"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "slot_participants_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
        ]
      }
      slot_resources: {
        Row: {
          assigned_at: string
          resource_id: string
          resource_type: string | null
          slot_id: string
        }
        Insert: {
          assigned_at?: string
          resource_id: string
          resource_type?: string | null
          slot_id: string
        }
        Update: {
          assigned_at?: string
          resource_id?: string
          resource_type?: string | null
          slot_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "slot_resources_slot_id_fkey"
            columns: ["slot_id"]
            isOneToOne: false
            referencedRelation: "slots"
            referencedColumns: ["id"]
          },
        ]
      }
      slots: {
        Row: {
          agenda_id: string
          capacity: number | null
          created_at: string
          currency: string | null
          end_at: string
          id: string
          metadata: Json | null
          participants_count: number
          payment_link: string | null
          price: number | null
          resource_id: string | null
          start_at: string
          status: Database["public"]["Enums"]["agenda_slot_status"]
          tags: string[] | null
          updated_at: string
        }
        Insert: {
          agenda_id: string
          capacity?: number | null
          created_at?: string
          currency?: string | null
          end_at: string
          id?: string
          metadata?: Json | null
          participants_count?: number
          payment_link?: string | null
          price?: number | null
          resource_id?: string | null
          start_at: string
          status?: Database["public"]["Enums"]["agenda_slot_status"]
          tags?: string[] | null
          updated_at?: string
        }
        Update: {
          agenda_id?: string
          capacity?: number | null
          created_at?: string
          currency?: string | null
          end_at?: string
          id?: string
          metadata?: Json | null
          participants_count?: number
          payment_link?: string | null
          price?: number | null
          resource_id?: string | null
          start_at?: string
          status?: Database["public"]["Enums"]["agenda_slot_status"]
          tags?: string[] | null
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "slots_agenda_id_fkey"
            columns: ["agenda_id"]
            isOneToOne: false
            referencedRelation: "agendas"
            referencedColumns: ["id"]
          },
        ]
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      commerce_invoice_status:
        | "draft"
        | "issued"
        | "paid_confirmed"
        | "cancelled"
        | "credited"
      commerce_payment_method:
        | "bank_transfer"
        | "cash"
        | "check"
        | "card_remote"
        | "other"
      commerce_quote_message_author_role: "requester" | "seller" | "admin"
      commerce_quote_request_status: "draft" | "submitted" | "qualified" | "cancelled"
      commerce_quote_status:
        | "draft"
        | "sent"
        | "viewed"
        | "accepted"
        | "rejected"
        | "expired"
        | "cancelled"
      agenda_slot_status:
        | "draft"
        | "pending"
        | "confirmed"
        | "paid"
        | "cancelled"
      booking_booking_status:
        | "requested"
        | "confirmed"
        | "cancelled_by_client"
        | "cancelled_by_provider"
        | "no_show"
        | "completed"
      booking_time_off_mode:
        | "block_slots"
        | "cancel_bookings"
        | "request_reschedule"
      consent_type:
        | "marketing"
        | "analytics"
        | "service"
        | "newsletter"
        | "third_party"
      user_role: "user" | "admin" | "super_admin"
      user_tier: "free" | "starter" | "pro" | "enterprise"
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}

type DatabaseWithoutInternals = Omit<Database, "__InternalSupabase">

type DefaultSchema = DatabaseWithoutInternals[Extract<keyof Database, "public">]

export type Tables<
  DefaultSchemaTableNameOrOptions extends
    | keyof (DefaultSchema["Tables"] & DefaultSchema["Views"])
    | { schema: keyof DatabaseWithoutInternals },
  TableName extends DefaultSchemaTableNameOrOptions extends {
    schema: keyof DatabaseWithoutInternals
  }
    ? keyof (DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Tables"] &
        DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Views"])
    : never = never,
> = DefaultSchemaTableNameOrOptions extends {
  schema: keyof DatabaseWithoutInternals
}
  ? (DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Tables"] &
      DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Views"])[TableName] extends {
      Row: infer R
    }
    ? R
    : never
  : DefaultSchemaTableNameOrOptions extends keyof (DefaultSchema["Tables"] &
        DefaultSchema["Views"])
    ? (DefaultSchema["Tables"] &
        DefaultSchema["Views"])[DefaultSchemaTableNameOrOptions] extends {
        Row: infer R
      }
      ? R
      : never
    : never

export type TablesInsert<
  DefaultSchemaTableNameOrOptions extends
    | keyof DefaultSchema["Tables"]
    | { schema: keyof DatabaseWithoutInternals },
  TableName extends DefaultSchemaTableNameOrOptions extends {
    schema: keyof DatabaseWithoutInternals
  }
    ? keyof DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Tables"]
    : never = never,
> = DefaultSchemaTableNameOrOptions extends {
  schema: keyof DatabaseWithoutInternals
}
  ? DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Tables"][TableName] extends {
      Insert: infer I
    }
    ? I
    : never
  : DefaultSchemaTableNameOrOptions extends keyof DefaultSchema["Tables"]
    ? DefaultSchema["Tables"][DefaultSchemaTableNameOrOptions] extends {
        Insert: infer I
      }
      ? I
      : never
    : never

export type TablesUpdate<
  DefaultSchemaTableNameOrOptions extends
    | keyof DefaultSchema["Tables"]
    | { schema: keyof DatabaseWithoutInternals },
  TableName extends DefaultSchemaTableNameOrOptions extends {
    schema: keyof DatabaseWithoutInternals
  }
    ? keyof DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Tables"]
    : never = never,
> = DefaultSchemaTableNameOrOptions extends {
  schema: keyof DatabaseWithoutInternals
}
  ? DatabaseWithoutInternals[DefaultSchemaTableNameOrOptions["schema"]]["Tables"][TableName] extends {
      Update: infer U
    }
    ? U
    : never
  : DefaultSchemaTableNameOrOptions extends keyof DefaultSchema["Tables"]
    ? DefaultSchema["Tables"][DefaultSchemaTableNameOrOptions] extends {
        Update: infer U
      }
      ? U
      : never
    : never

export type Enums<
  DefaultSchemaEnumNameOrOptions extends
    | keyof DefaultSchema["Enums"]
    | { schema: keyof DatabaseWithoutInternals },
  EnumName extends DefaultSchemaEnumNameOrOptions extends {
    schema: keyof DatabaseWithoutInternals
  }
    ? keyof DatabaseWithoutInternals[DefaultSchemaEnumNameOrOptions["schema"]]["Enums"]
    : never = never,
> = DefaultSchemaEnumNameOrOptions extends {
  schema: keyof DatabaseWithoutInternals
}
  ? DatabaseWithoutInternals[DefaultSchemaEnumNameOrOptions["schema"]]["Enums"][EnumName]
  : DefaultSchemaEnumNameOrOptions extends keyof DefaultSchema["Enums"]
    ? DefaultSchema["Enums"][DefaultSchemaEnumNameOrOptions]
    : never

export type CompositeTypes<
  PublicCompositeTypeNameOrOptions extends
    | keyof DefaultSchema["CompositeTypes"]
    | { schema: keyof DatabaseWithoutInternals },
  CompositeTypeName extends PublicCompositeTypeNameOrOptions extends {
    schema: keyof DatabaseWithoutInternals
  }
    ? keyof DatabaseWithoutInternals[PublicCompositeTypeNameOrOptions["schema"]]["CompositeTypes"]
    : never = never,
> = PublicCompositeTypeNameOrOptions extends {
  schema: keyof DatabaseWithoutInternals
}
  ? DatabaseWithoutInternals[PublicCompositeTypeNameOrOptions["schema"]]["CompositeTypes"][CompositeTypeName]
  : PublicCompositeTypeNameOrOptions extends keyof DefaultSchema["CompositeTypes"]
    ? DefaultSchema["CompositeTypes"][PublicCompositeTypeNameOrOptions]
    : never

// Helpers types pour usage simplifié
export type Profile = Tables<'profiles'>
export type ProfileInsert = TablesInsert<'profiles'>
export type ProfileUpdate = TablesUpdate<'profiles'>

export type UserConsent = Tables<'user_consents'>
export type UserConsentInsert = TablesInsert<'user_consents'>
export type UserConsentUpdate = TablesUpdate<'user_consents'>

export type Category = Tables<'categories'>
export type CategoryInsert = TablesInsert<'categories'>
export type CategoryUpdate = TablesUpdate<'categories'>

export type UserCategoryPreference = Tables<'user_category_preferences'>
export type UserCategoryPreferenceInsert = TablesInsert<'user_category_preferences'>
export type UserCategoryPreferenceUpdate = TablesUpdate<'user_category_preferences'>

export type UserRole = Enums<'user_role'>
export type UserTier = Enums<'user_tier'>
export type ConsentType = Enums<'consent_type'>
export type AgendaSlotStatus = Enums<'agenda_slot_status'>
export type BookingBookingStatus = Enums<'booking_booking_status'>
export type BookingTimeOffMode = Enums<'booking_time_off_mode'>

export const Constants = {
  public: {
    Enums: {
      commerce_invoice_status: [
        "draft",
        "issued",
        "paid_confirmed",
        "cancelled",
        "credited",
      ],
      commerce_payment_method: [
        "bank_transfer",
        "cash",
        "check",
        "card_remote",
        "other",
      ],
      commerce_quote_message_author_role: ["requester", "seller", "admin"],
      commerce_quote_request_status: ["draft", "submitted", "qualified", "cancelled"],
      commerce_quote_status: [
        "draft",
        "sent",
        "viewed",
        "accepted",
        "rejected",
        "expired",
        "cancelled",
      ],
      agenda_slot_status: [
        "draft",
        "pending",
        "confirmed",
        "paid",
        "cancelled",
      ],
      booking_booking_status: [
        "requested",
        "confirmed",
        "cancelled_by_client",
        "cancelled_by_provider",
        "no_show",
        "completed",
      ],
      booking_time_off_mode: [
        "block_slots",
        "cancel_bookings",
        "request_reschedule",
      ],
      consent_type: [
        "marketing",
        "analytics",
        "service",
        "newsletter",
        "third_party",
      ],
      user_role: ["user", "admin", "super_admin"],
      user_tier: ["free", "starter", "pro", "enterprise"],
    },
  },
} as const
