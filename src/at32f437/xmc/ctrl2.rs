#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CA` reader - Number of column address bits"]
pub type CA_R = crate::FieldReader;
#[doc = "Field `CA` writer - Number of column address bits"]
pub type CA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RA` reader - Number of row address bits"]
pub type RA_R = crate::FieldReader;
#[doc = "Field `RA` writer - Number of row address bits"]
pub type RA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DB` reader - Memory data bus width"]
pub type DB_R = crate::FieldReader;
#[doc = "Field `DB` writer - Memory data bus width"]
pub type DB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INBK` reader - Number of internal banks"]
pub type INBK_R = crate::BitReader;
#[doc = "Field `INBK` writer - Number of internal banks"]
pub type INBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAS` reader - CAS latency"]
pub type CAS_R = crate::FieldReader;
#[doc = "Field `CAS` writer - CAS latency"]
pub type CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRP` reader - Write protection"]
pub type WRP_R = crate::BitReader;
#[doc = "Field `WRP` writer - Write protection"]
pub type WRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - Clock division configuration"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock division configuration"]
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BSTR` reader - Burst read"]
pub type BSTR_R = crate::BitReader;
#[doc = "Field `BSTR` writer - Burst read"]
pub type BSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Read pipe"]
pub type RD_R = crate::FieldReader;
#[doc = "Field `RD` writer - Read pipe"]
pub type RD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    pub fn ca(&self) -> CA_R {
        CA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn db(&self) -> DB_R {
        DB_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    pub fn inbk(&self) -> INBK_R {
        INBK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Clock division configuration"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn bstr(&self) -> BSTR_R {
        BSTR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Read pipe"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("ca", &self.ca())
            .field("ra", &self.ra())
            .field("db", &self.db())
            .field("inbk", &self.inbk())
            .field("cas", &self.cas())
            .field("wrp", &self.wrp())
            .field("clkdiv", &self.clkdiv())
            .field("bstr", &self.bstr())
            .field("rd", &self.rd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CA_W<CTRL2_SPEC> {
        CA_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<CTRL2_SPEC> {
        RA_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn db(&mut self) -> DB_W<CTRL2_SPEC> {
        DB_W::new(self, 4)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    #[must_use]
    pub fn inbk(&mut self) -> INBK_W<CTRL2_SPEC> {
        INBK_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CAS_W<CTRL2_SPEC> {
        CAS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WRP_W<CTRL2_SPEC> {
        WRP_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Clock division configuration"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CTRL2_SPEC> {
        CLKDIV_W::new(self, 10)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    #[must_use]
    pub fn bstr(&mut self) -> BSTR_W<CTRL2_SPEC> {
        BSTR_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Read pipe"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<CTRL2_SPEC> {
        RD_W::new(self, 13)
    }
}
#[doc = "SDRAM Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x02d0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x02d0;
}
