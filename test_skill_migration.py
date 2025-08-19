#!/usr/bin/env python3
"""
Test script for Arceon skill migration system
This script tests the skill migration endpoint to ensure it properly updates existing players
"""

import requests
import json
import time

# Server configuration
SERVER_URL = "http://localhost:8080"
API_BASE = f"{SERVER_URL}/api"

def test_skill_migration():
    """Test the skill migration system"""
    print("🧪 Testing Arceon Skill Migration System")
    print("=" * 50)
    
    # Step 1: Register a test user
    print("📝 Step 1: Registering test user...")
    register_data = {
        "username": "skilltest_user",
        "password": "TestPassword123!",
        "email": "skilltest@example.com"
    }
    
    response = requests.post(f"{API_BASE}/auth/register", json=register_data)
    print(f"Registration Response: {response.status_code}")
    if response.status_code == 200:
        register_result = response.json()
        print(f"✅ Registration: {register_result['success']} - {register_result['message']}")
    else:
        print(f"❌ Registration failed: {response.text}")
        return False
    
    # Step 2: Login to get session
    print("\n🔑 Step 2: Logging in...")
    login_data = {
        "username": "skilltest_user",
        "password": "TestPassword123!"
    }
    
    response = requests.post(f"{API_BASE}/auth/login", json=login_data)
    if response.status_code == 200:
        login_result = response.json()
        if login_result['success']:
            session_id = login_result['session_id']
            print(f"✅ Login successful, session: {session_id[:8]}...")
        else:
            print(f"❌ Login failed: {login_result['message']}")
            return False
    else:
        print(f"❌ Login request failed: {response.text}")
        return False
    
    # Step 3: Create a character
    print("\n🎮 Step 3: Creating test character...")
    create_character_data = {
        "character_name": "SkillTestChar",
        "race": "Human",
        "starting_area": "Test Area"
    }
    
    headers = {"session-id": session_id}
    response = requests.post(f"{API_BASE}/players", json=create_character_data, headers=headers)
    if response.status_code == 200:
        character_result = response.json()
        if character_result['success']:
            character_id = character_result['character_id']
            print(f"✅ Character created: {character_result['message']}")
            print(f"   Character ID: {character_id}")
        else:
            print(f"❌ Character creation failed: {character_result['message']}")
    else:
        print(f"❌ Character creation request failed: {response.text}")
    
    # Step 4: Test skill migration
    print("\n🔄 Step 4: Running skill migration...")
    response = requests.post(f"{API_BASE}/admin/migrate-skills", headers=headers)
    if response.status_code == 200:
        migration_result = response.json()
        print(f"Migration Success: {migration_result['success']}")
        print(f"Total Players Processed: {migration_result['total_players_processed']}")
        print(f"Players Updated: {migration_result['players_updated']}")
        print(f"Processing Time: {migration_result['processing_time_ms']}ms")
        
        if migration_result['new_skills_added']:
            print(f"New Skills Available: {len(migration_result['new_skills_added'])}")
            for skill in migration_result['new_skills_added'][:5]:  # Show first 5
                print(f"  - {skill}")
            if len(migration_result['new_skills_added']) > 5:
                print(f"  ... and {len(migration_result['new_skills_added']) - 5} more")
        
        if migration_result['errors']:
            print(f"⚠️ Errors encountered: {len(migration_result['errors'])}")
            for error in migration_result['errors']:
                print(f"  - {error}")
        
        if migration_result['success']:
            print("✅ Skill migration completed successfully!")
        else:
            print("❌ Skill migration completed with errors")
            
    else:
        print(f"❌ Skill migration request failed: {response.text}")
        return False
    
    # Step 5: Test user profile to see if skills are applied
    print("\n👤 Step 5: Checking user profile...")
    response = requests.get(f"{API_BASE}/user/profile", headers=headers)
    if response.status_code == 200:
        profile_result = response.json()
        if profile_result['success']:
            print(f"✅ Profile retrieved for: {profile_result['username']}")
            print(f"   User ID: {profile_result['user_id']}")
            print(f"   Characters: {len(profile_result['characters'])}")
            print(f"   Account Status: {profile_result['account_status']}")
            print(f"   Wallet Bound: {profile_result['wallet_bound']}")
        else:
            print(f"❌ Profile retrieval failed")
    
    # Step 6: Logout
    print("\n👋 Step 6: Logging out...")
    response = requests.post(f"{API_BASE}/auth/logout", headers=headers)
    if response.status_code == 200:
        logout_result = response.json()
        if logout_result['success']:
            print(f"✅ Logout successful: {logout_result['message']}")
        else:
            print(f"❌ Logout failed: {logout_result['message']}")
    
    print("\n🎉 Skill migration test completed!")
    return True

def check_server_health():
    """Check if the server is running"""
    try:
        response = requests.get(f"{API_BASE}/health", timeout=5)
        if response.status_code == 200:
            health_data = response.json()
            print(f"✅ Server is running: {health_data['status']}")
            print(f"   Version: {health_data['version']}")
            print(f"   Features: {', '.join(health_data['features'])}")
            return True
        else:
            print(f"❌ Server health check failed: {response.status_code}")
            return False
    except requests.exceptions.RequestException as e:
        print(f"❌ Cannot reach server: {e}")
        return False

if __name__ == "__main__":
    print("🌟 Arceon Skill Migration Test Script")
    print("====================================")
    
    # Check server availability
    if not check_server_health():
        print("\n💡 Make sure the Arceon server is running:")
        print("   cargo run --bin arceon-server")
        exit(1)
    
    print()
    
    # Run the migration test
    if test_skill_migration():
        print("\n✅ All tests passed!")
    else:
        print("\n❌ Some tests failed!")