#[doc = "Register `OVSP` reader"]
pub type R = crate::R<OVSP_SPEC>;
#[doc = "Register `OVSP` writer"]
pub type W = crate::W<OVSP_SPEC>;
#[doc = "Field `OOSEN` reader - Ordinary oversampling enable"]
pub type OOSEN_R = crate::BitReader;
#[doc = "Field `OOSEN` writer - Ordinary oversampling enable"]
pub type OOSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSEN` reader - Preempted oversampling enable"]
pub type POSEN_R = crate::BitReader;
#[doc = "Field `POSEN` writer - Preempted oversampling enable"]
pub type POSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSRSEL` reader - Oversampling ratio select"]
pub type OSRSEL_R = crate::FieldReader;
#[doc = "Field `OSRSEL` writer - Oversampling ratio select"]
pub type OSRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSSSEL` reader - Oversampling shift select"]
pub type OSSSEL_R = crate::FieldReader;
#[doc = "Field `OSSSEL` writer - Oversampling shift select"]
pub type OSSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OOSTREN` reader - Ordinary oversampling trigger mode enable"]
pub type OOSTREN_R = crate::BitReader;
#[doc = "Field `OOSTREN` writer - Ordinary oversampling trigger mode enable"]
pub type OOSTREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OOSRSEL` reader - Ordinary oversampling recovery mode select"]
pub type OOSRSEL_R = crate::BitReader;
#[doc = "Field `OOSRSEL` writer - Ordinary oversampling recovery mode select"]
pub type OOSRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ordinary oversampling enable"]
    #[inline(always)]
    pub fn oosen(&self) -> OOSEN_R {
        OOSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Preempted oversampling enable"]
    #[inline(always)]
    pub fn posen(&self) -> POSEN_R {
        POSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio select"]
    #[inline(always)]
    pub fn osrsel(&self) -> OSRSEL_R {
        OSRSEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift select"]
    #[inline(always)]
    pub fn osssel(&self) -> OSSSEL_R {
        OSSSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Ordinary oversampling trigger mode enable"]
    #[inline(always)]
    pub fn oostren(&self) -> OOSTREN_R {
        OOSTREN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Ordinary oversampling recovery mode select"]
    #[inline(always)]
    pub fn oosrsel(&self) -> OOSRSEL_R {
        OOSRSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OVSP")
            .field("oosrsel", &self.oosrsel())
            .field("oostren", &self.oostren())
            .field("osssel", &self.osssel())
            .field("osrsel", &self.osrsel())
            .field("posen", &self.posen())
            .field("oosen", &self.oosen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Ordinary oversampling enable"]
    #[inline(always)]
    pub fn oosen(&mut self) -> OOSEN_W<'_, OVSP_SPEC> {
        OOSEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Preempted oversampling enable"]
    #[inline(always)]
    pub fn posen(&mut self) -> POSEN_W<'_, OVSP_SPEC> {
        POSEN_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Oversampling ratio select"]
    #[inline(always)]
    pub fn osrsel(&mut self) -> OSRSEL_W<'_, OVSP_SPEC> {
        OSRSEL_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift select"]
    #[inline(always)]
    pub fn osssel(&mut self) -> OSSSEL_W<'_, OVSP_SPEC> {
        OSSSEL_W::new(self, 5)
    }
    #[doc = "Bit 9 - Ordinary oversampling trigger mode enable"]
    #[inline(always)]
    pub fn oostren(&mut self) -> OOSTREN_W<'_, OVSP_SPEC> {
        OOSTREN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Ordinary oversampling recovery mode select"]
    #[inline(always)]
    pub fn oosrsel(&mut self) -> OOSRSEL_W<'_, OVSP_SPEC> {
        OOSRSEL_W::new(self, 10)
    }
}
#[doc = "oversampling register\n\nYou can [`read`](crate::Reg::read) this register and get [`ovsp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovsp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVSP_SPEC;
impl crate::RegisterSpec for OVSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsp::R`](R) reader structure"]
impl crate::Readable for OVSP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ovsp::W`](W) writer structure"]
impl crate::Writable for OVSP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVSP to value 0"]
impl crate::Resettable for OVSP_SPEC {}
