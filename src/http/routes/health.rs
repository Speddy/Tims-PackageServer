// Copyright (C) 2013 - 2021 Tim Düsterhus
// Copyright (C) 2021 Maximilian Mader
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{http::error::Error::PackageListUnavailable, PACKAGE_LIST};
use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective},
    HttpRequest, HttpResponse, Responder,
};

#[get("/health")]
pub async fn health(req: HttpRequest) -> impl Responder {
    PACKAGE_LIST
        .load_full()
        .map(|_| {
            HttpResponse::Ok()
                .insert_header(CacheControl(vec![
                    CacheDirective::NoCache,
                    CacheDirective::NoStore,
                    CacheDirective::Private,
                ]))
                .body("Ok")
        })
        .ok_or(PackageListUnavailable(req))
}
